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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:18 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SNC registers"]
unsafe impl ::core::marker::Send for super::Snc {}
unsafe impl ::core::marker::Sync for super::Snc {}
impl super::Snc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Sensor Node Control Register"]
    #[inline(always)]
    pub const fn snc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Sensor Node Low-Power Timer Register"]
    #[inline(always)]
    pub const fn snc_lp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncLpTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncLpTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Sensor Node Program Counter"]
    #[inline(always)]
    pub const fn snc_pc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncPcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncPcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Sensor Node core - Operand 1 Register"]
    #[inline(always)]
    pub const fn snc_r1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncR1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncR1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Sensor Node core - Operand 2 Register"]
    #[inline(always)]
    pub const fn snc_r2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncR2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncR2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Sensor Node Status Register"]
    #[inline(always)]
    pub const fn snc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Sensor Node core - Temporary Register 1"]
    #[inline(always)]
    pub const fn snc_tmp1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncTmp1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncTmp1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Sensor Node core - Temporary Register 2"]
    #[inline(always)]
    pub const fn snc_tmp2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SncTmp2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SncTmp2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncCtrlReg_SPEC;
impl crate::sealed::RegSpec for SncCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node Control Register"]
pub type SncCtrlReg = crate::RegValueT<SncCtrlReg_SPEC>;

impl SncCtrlReg {
    #[doc = "When set, the specific bit-field auto-clears the SNC_IRQ_EN field, if the latter is already set. By this way, the IRQ line towards either the CM33 and/or the PDC is cleared. Hence, the CM33 should set this bit-field as soon as it captures the interrupt from the Sensor Node.\nNote: Any SW writes to this bit-field will be discarded if the SNC_IRQ_EN bit-field is not set. It is finally noted that the SNC_IRQ_ACK bit-field is also auto-clear and it is de-asserted together with SNC_IRQ_EN."]
    #[inline(always)]
    pub fn snc_irq_ack(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The specific bit-field determines if the IRQ line of the Sensor Node will be routed towards either the host processor (CM33) or the Power Domains Controller (PDC), or to both of them, according to the following configuration:\n0x0 = Neither the CM33 nor the PDC are triggered, both IRQ lines are low regardless of the value of SNC_IRQ_EN bit-field.\n0x1 = CM33 should be triggered, provided that SNC_IRQ_EN is set\n0x2 = PDC should be triggered, provided that SNC_IRQ_EN is set\n0x3 = Both CM33 and PDC should be triggered, provided that SNC_IRQ_EN is set\nNote: It must be noted that the specific bit-field is locked after set the SNC_IRQ_EN field of the same register. Hence, the SNC IRQ configuration cannot be changed after the IRQ bit-field is set and before the IRQ is acknowledged (by CM33). It is also noted that after having set SNC_IRQ_EN via SW, the specific bit-field can be de-asserted only by setting the SNC_IRQ_ACK bit-field (see also the description of this bit-field, also residing in SNC_CTRL_REG)."]
    #[inline(always)]
    pub fn snc_irq_config(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, the specific bit-field may generate a (level-sensitive) IRQ to trigger either the host processor (CM33) or the Power Domains Controller (PDC) or both, depending on the configuration set in the SNC_IRQ_CONFIG bit-field of SNC_CTRL_REG. As soon as the SNC_IRQ_EN is set, it can be cleared only by setting the SNC_IRQ_ACK bit-field."]
    #[inline(always)]
    pub fn snc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it clears the value of the counter used in the Sensor Node\'s branch command (COBR), when performing an iterative branch of up to 128 times. This bit-field is auto-cleared with the next SNC clock."]
    #[inline(always)]
    pub fn snc_branch_loop_init(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is the Sensor Node Controller\'s synchronous clear bit-field. When set, it resets the state of the Sensor Node Controller and sets back its program counter (SNC_PC_REG) to the programmed base address, as determined by SNC_BASE_REG register (located in memory controller). This bit-field is auto-cleared with the next SNC clock.\nNote: Setting this bit-field may interrupt the Sensor Node\'s regular execution and any command currently being exeucuted may be abnormally terminated."]
    #[inline(always)]
    pub fn snc_reset(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the detection of system bus errors that may occur in case a non-mapped address is used by the Sensor Node controller, when performing a register access.\nNote: In case of a bus error detection, the Sensor Node will set to \'1\' the BUS_ERROR_STATUS bit-field of SNC_STATUS_REG and will continue normally to the next command."]
    #[inline(always)]
    pub fn bus_error_detect_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field bypasses the enable of Sensor Node that comes from the PDC. In this mode, the Sensor Node can be started and stopped via the SNC_EN bit-field of SNC_CTRL_REG.\nNote: This mode is suggested to be used for debugging purposes. Älso, the base address of the Sensor Node should have been programmed to the target value, before this bit-field is set."]
    #[inline(always)]
    pub fn snc_sw_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sensor Node Controller\'s enable bit-field. When set, it may activate the Sensor Node, provided that the SNC_SW_CTRL bit-field is also set. If not, then the specific bit-field is not effective and Sensor Node\'s actual enable is controller by the Power Domains Controller (PDC).\nNote: When SNC_SW_CTRL bit-field is set, the Sensor Node is controlled by the user. Thus, in that mode, it can be started and stopped by setting and resetting the SNC_EN field. When SNC_EN is reset, the Sensor Node will first complete the last on-going command before being halted."]
    #[inline(always)]
    pub fn snc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncCtrlReg {
    #[inline(always)]
    fn default() -> SncCtrlReg {
        <crate::RegValueT<SncCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncLpTimerReg_SPEC;
impl crate::sealed::RegSpec for SncLpTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node Low-Power Timer Register"]
pub type SncLpTimerReg = crate::RegValueT<SncLpTimerReg_SPEC>;

impl SncLpTimerReg {
    #[doc = "This bit-field returns the current value of the Sensor Node\'s 8-bit timer, running with the low-power clock and may be used for debugging purposes. The specific timer is used to implement a delay of up to 256 ticks of the low-power clock."]
    #[inline(always)]
    pub fn lp_timer(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, SncLpTimerReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, SncLpTimerReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncLpTimerReg {
    #[inline(always)]
    fn default() -> SncLpTimerReg {
        <crate::RegValueT<SncLpTimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncPcReg_SPEC;
impl crate::sealed::RegSpec for SncPcReg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node Program Counter"]
pub type SncPcReg = crate::RegValueT<SncPcReg_SPEC>;

impl SncPcReg {
    #[doc = "This bit-field returns the Sensor Node\'s program counter bits \\[18:2\\], which at the same time is the program counter\'s offset from the starting address of SYSRAM (0x20.000.000), and it is can be set by the user, as soon as Sensor Node has been stopped.\nThe 13 MSBs of the program counter are tied to \'0x400\', since the Sensor Node always executes from SYSRAM, while its 2 LSBs are always tied to \'0\', since memory accesses are always of 32-bit.\nNOTE: The Sensor Node can be stopped by clearing the SNC_EN bit-field of SNC_CTRL_REG and provided that the Power Domains Controller (PDC) is bypassed. The latter can be done by setting to \'1\' the SNC_SW_CTRL bit-field of SNC_CTRL_REG."]
    #[inline(always)]
    pub fn pc_reg(
        self,
    ) -> crate::common::RegisterField<2, 0x1ffff, 1, 0, u32, SncPcReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1ffff,1,0,u32, SncPcReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncPcReg {
    #[inline(always)]
    fn default() -> SncPcReg {
        <crate::RegValueT<SncPcReg_SPEC> as RegisterValue<_>>::new(536870912)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncR1Reg_SPEC;
impl crate::sealed::RegSpec for SncR1Reg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node core - Operand 1 Register"]
pub type SncR1Reg = crate::RegValueT<SncR1Reg_SPEC>;

impl SncR1Reg {
    #[doc = "Returns the current value of the first 32-bit of the last SNC command executed."]
    #[inline(always)]
    pub fn r1_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SncR1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, SncR1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncR1Reg {
    #[inline(always)]
    fn default() -> SncR1Reg {
        <crate::RegValueT<SncR1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncR2Reg_SPEC;
impl crate::sealed::RegSpec for SncR2Reg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node core - Operand 2 Register"]
pub type SncR2Reg = crate::RegValueT<SncR2Reg_SPEC>;

impl SncR2Reg {
    #[doc = "Returns the current value of the second 32-bit word of the last SNC command executed. This is useful for the SNC commands composed by two 32-bit words."]
    #[inline(always)]
    pub fn r2_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SncR2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, SncR2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncR2Reg {
    #[inline(always)]
    fn default() -> SncR2Reg {
        <crate::RegValueT<SncR2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncStatusReg_SPEC;
impl crate::sealed::RegSpec for SncStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node Status Register"]
pub type SncStatusReg = crate::RegValueT<SncStatusReg_SPEC>;

impl SncStatusReg {
    #[doc = "0 : Sensor node\'s program counter is controlled by the Sensor Node\'s FSM, incemented by 4 after the fetching of each 32-bit command word.\n1 : Sensor node\'s program counter is loaded with a new value. The assertion of this signal requires the Sensor Node to have been first stopped, so the user must first check that the SNC_IS_STOPPED bit-field of this register is asserted, before writing the program counter. The SNC_PC_LOADED bit-field is auto-clear and it is reset to \'0\' as soon as the user has re-started the Sensor Node.\nNote: To start and stop the Sensor Node manually, the SNC_SW_CTRL and SNC_EN bit-fields of SNC_CTRL_REG must have been set by the user. This mode of operation is bypassing the Power Domains Controller and it is to be used for debugging purposes."]
    #[inline(always)]
    pub fn snc_pc_loaded(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 : Sensor Node is operational and its FSM is running.\n1 : Sensor Node is stopped and its FSM is halted.To leave this state, the SNC_EN bit-field of SNC_CTRL_REG must be set, provided that the SNC_SW_CTRL bit-field of the same register is also set. This mode is used for debugging purposes, bypassing the enable of SNC coming from the Power Domains Controller.\nNote: The SNC_PC_REG register can be modified by SW if and only if the SNC_IS_STOPPED bit is set. Otherwise, the writes to SNC_PC_REG are discarded."]
    #[inline(always)]
    pub fn snc_is_stopped(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 : No opcode error has occurred, Sensor Node continues normally.\n1 : An opcode error has occurred. Sensor Node will continue its execution, but will set also the specific bit-field to \'1\', for debugging purposes.\nNote: After being set, this bit-field will be cleared only when the Sensor Node is re-initialized, by starting again from its base address. The latter can happen either by activating the SNC_RESET bit-field of SNC_CTRL_REG or by stopping and starting again the Sensor Node. This is possible only when the PDC is bypassed, so when the Sensor Node is controlled by SNC_EN and SNC_SW_CTRL bit-fields of SNC_CTRL_REG."]
    #[inline(always)]
    pub fn hard_fault_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 : No system bus error detected, Sensor Node continues normally\n1 : Bus error occurred. Sensor Node will continue, but it will also set the specific flag, which can be used for debugging purposes.\nNote: This bit-field will be reset to \'0\' only when the Sensor Node is re-initialized, by starting again from its base address."]
    #[inline(always)]
    pub fn bus_error_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 : Sensor Node has not yet completed the target program\'s execution.\n1 : Sensor Node has completed the targer program\'s execution. Together with the update of the status bit, a pulse is also generated to notify the PDC that the Sensor Node is done.\nNote: This bit-field is set only when the \"SLP\" (sleep) command is executed, which should be issued after the completion of all pending tasks of the Sensor Node. It will be reset to \'0\' only when the Sensor Node re-starts, by executing from the base address.\nThis can be done by either toggling (de-asserting and re-asserting again) the SNC_EN bit-field of SNC_CTRL_REG, if the SNC is controlled by SW, or by just re-setting the SNC state via the SNC_RESET bit-field of the same register."]
    #[inline(always)]
    pub fn snc_done_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SncStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SncStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sensor Node\'s \'GR\' (greater) flag. It can be modified either by the Sensor Node\'s core (by executing an \"RDCGR\" command) or by the Sensor Node\'s microcode, when the latter directly modifies the specific bit-field of SNC_STATUS_REG.\nWhen the Sensor Node\'s FSM is in its initial state (which may happen either by switching-on the PD_COM power domain or by resetting the Sensor Node via SNC_CTRL_REG.SNC_RESET), the specific bit-field is kept to \'0\', for initialization purposes.\nWhen the Sensor Node is stopped and then reset, the Sensor Node\'s FSM is not in its initial state and in that case, the GR_FLAG bit-field should be reset by the user (if the application needs this to be initialized to \'0\'). Otherwise, it can be left as it is, until being updated by the Sensor Node itself (upon executing an \"RDCGR\" command).\nIn general, however, this bit-field should not be modified by either the host processor (CM33) or the CMAC processor (CM0+), and especially when the Sensor Node is enabled and operational."]
    #[inline(always)]
    pub fn gr_flag(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SncStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SncStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sensor Node\'s \'EQ\' (equalhigh) flag. It can be modified either by the Sensor Node\'s core (by executing an \"RDCBI\" command) or by the Sensor Node\'s microcode, when the latter directly modifies the specific bit-field of SNC_STATUS_REG.\nWhen the Sensor Node\'s FSM is in its initial state (which may happen either by switching-on the PD_COM power domain or by resetting the Sensor Node via SNC_CTRL_REG.SNC_RESET), the specific bit-field is kept to \'0\', for initialization purposes.\nWhen the Sensor Node is stopped and then reset, the Sensor Node\'s FSM is not in its initial state and in that case, the EQ_FLAG bit-field should be reset by the user (if the application needs this to be initialized to \'0\'). Otherwise, it can be left as it is, until being updated by the Sensor Node itself (upon executing an \"RDCBI\" command).\nIn general, however, this bit-field should not be modified by either the host processor (CM33) or the CMAC processor (CM0+), and especially when the Sensor Node is enabled and operational."]
    #[inline(always)]
    pub fn eq_flag(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SncStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SncStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SncStatusReg {
    #[inline(always)]
    fn default() -> SncStatusReg {
        <crate::RegValueT<SncStatusReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncTmp1Reg_SPEC;
impl crate::sealed::RegSpec for SncTmp1Reg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node core - Temporary Register 1"]
pub type SncTmp1Reg = crate::RegValueT<SncTmp1Reg_SPEC>;

impl SncTmp1Reg {
    #[doc = "Returns the current value of the Sensor Node\'s first temporary register. To be used for debugging purposes."]
    #[inline(always)]
    pub fn tmp1_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SncTmp1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, SncTmp1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncTmp1Reg {
    #[inline(always)]
    fn default() -> SncTmp1Reg {
        <crate::RegValueT<SncTmp1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SncTmp2Reg_SPEC;
impl crate::sealed::RegSpec for SncTmp2Reg_SPEC {
    type DataType = u32;
}
#[doc = "Sensor Node core - Temporary Register 2"]
pub type SncTmp2Reg = crate::RegValueT<SncTmp2Reg_SPEC>;

impl SncTmp2Reg {
    #[doc = "Returns the current value of the Sensor Node\'s second temporary register. To be used for debugging purposes."]
    #[inline(always)]
    pub fn tmp2_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SncTmp2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, SncTmp2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SncTmp2Reg {
    #[inline(always)]
    fn default() -> SncTmp2Reg {
        <crate::RegValueT<SncTmp2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
