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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UART registers"]
unsafe impl ::core::marker::Send for super::Uart {}
unsafe impl ::core::marker::Sync for super::Uart {}
impl super::Uart {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Component Parameter Register"]
    #[inline(always)]
    pub const fn uart_cpr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartCprReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartCprReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "Component Type Register"]
    #[inline(always)]
    pub const fn uart_ctr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartCtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartCtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[doc = "Divisor Latch Fraction Register"]
    #[inline(always)]
    pub const fn uart_dlf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDlfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDlfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn uart_dmasa_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDmasaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDmasaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register/Divisor Latch High"]
    #[inline(always)]
    pub const fn uart_ier_dlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIerDlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIerDlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Identification Register/FIFO Control Register"]
    #[inline(always)]
    pub const fn uart_iir_fcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIirFcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIirFcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Line Control Register"]
    #[inline(always)]
    pub const fn uart_lcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Line Status Register"]
    #[inline(always)]
    pub const fn uart_lsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Modem Control Register"]
    #[inline(always)]
    pub const fn uart_mcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartMcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartMcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
    #[inline(always)]
    pub const fn uart_rbr_thr_dll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartRbrThrDllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartRbrThrDllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Shadow Break Control Register"]
    #[inline(always)]
    pub const fn uart_sbcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSbcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSbcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Scratchpad Register"]
    #[inline(always)]
    pub const fn uart_scr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartScrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartScrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Software Reset Register."]
    #[inline(always)]
    pub const fn uart_srr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Component Version"]
    #[inline(always)]
    pub const fn uart_ucv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUcvReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUcvReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[doc = "UART Status register."]
    #[inline(always)]
    pub const fn uart_usr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCprReg_SPEC;
impl crate::sealed::RegSpec for UartCprReg_SPEC {
    type DataType = u16;
}

#[doc = "Component Parameter Register"]
pub type UartCprReg = crate::RegValueT<UartCprReg_SPEC>;

impl UartCprReg {
    #[doc = "Component Parameter Register"]
    #[inline(always)]
    pub fn cpr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, UartCprReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,UartCprReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartCprReg {
    #[inline(always)]
    fn default() -> UartCprReg {
        <crate::RegValueT<UartCprReg_SPEC> as RegisterValue<_>>::new(14657)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCtrReg_SPEC;
impl crate::sealed::RegSpec for UartCtrReg_SPEC {
    type DataType = u32;
}

#[doc = "Component Type Register"]
pub type UartCtrReg = crate::RegValueT<UartCtrReg_SPEC>;

impl UartCtrReg {
    #[doc = "Component Type Register"]
    #[inline(always)]
    pub fn ctr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartCtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartCtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartCtrReg {
    #[inline(always)]
    fn default() -> UartCtrReg {
        <crate::RegValueT<UartCtrReg_SPEC> as RegisterValue<_>>::new(272)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDlfReg_SPEC;
impl crate::sealed::RegSpec for UartDlfReg_SPEC {
    type DataType = u16;
}

#[doc = "Divisor Latch Fraction Register"]
pub type UartDlfReg = crate::RegValueT<UartDlfReg_SPEC>;

impl UartDlfReg {
    #[doc = "The fractional value is added to integer value set by DLH, DLL. Fractional value is determined by (Divisor Fraction value)/(2^DLF_SIZE)."]
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UartDlfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UartDlfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDlfReg {
    #[inline(always)]
    fn default() -> UartDlfReg {
        <crate::RegValueT<UartDlfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDmasaReg_SPEC;
impl crate::sealed::RegSpec for UartDmasaReg_SPEC {
    type DataType = u16;
}

#[doc = "DMA Software Acknowledge"]
pub type UartDmasaReg = crate::RegValueT<UartDmasaReg_SPEC>;

impl UartDmasaReg {
    #[doc = "This register is use to perform DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This will cause the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn dmasa(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartDmasaReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartDmasaReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDmasaReg {
    #[inline(always)]
    fn default() -> UartDmasaReg {
        <crate::RegValueT<UartDmasaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIerDlhReg_SPEC;
impl crate::sealed::RegSpec for UartIerDlhReg_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Enable Register/Divisor Latch High"]
pub type UartIerDlhReg = crate::RegValueT<UartIerDlhReg_SPEC>;

impl UartIerDlhReg {
    #[doc = "Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. \nDivisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn dlh6_4(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, UartIerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn elsi_dhl2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled \nDivisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO\'s enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn erbfi_dlh0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartIerDlhReg {
    #[inline(always)]
    fn default() -> UartIerDlhReg {
        <crate::RegValueT<UartIerDlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIirFcrReg_SPEC;
impl crate::sealed::RegSpec for UartIirFcrReg_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Identification Register/FIFO Control Register"]
pub type UartIirFcrReg = crate::RegValueT<UartIirFcrReg_SPEC>;

impl UartIirFcrReg {
    #[doc = "Interrupt Identification Register: Bits\\[7:6\\], returns 00. Bits\\[3:0\\], Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout."]
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, UartIirFcrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            UartIirFcrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartIirFcrReg {
    #[inline(always)]
    fn default() -> UartIirFcrReg {
        <crate::RegValueT<UartIirFcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLcrReg_SPEC;
impl crate::sealed::RegSpec for UartLcrReg_SPEC {
    type DataType = u16;
}

#[doc = "Line Control Register"]
pub type UartLcrReg = crate::RegValueT<UartLcrReg_SPEC>;

impl UartLcrReg {
    #[doc = "Divisor Latch Access Bit.\nThis bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART.\nThis bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Break Control Bit.\nThis is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\] set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Even Parity Select. Writeable only when UART is not busy (USR\\[0\\] is zero).\nThis is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Parity Enable. Writeable only when UART is not busy (USR\\[0\\] is zero).\nThis bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively.\n0 = parity disabled\n1 = parity enabled"]
    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of stop bits.\nThis is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data.\nIf set to one and the data bits are set to 5 (LCR\\[1:0\\] set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit.\n0 = 1 stop bit\n1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Length Select.\nThis is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows:\n00 = 5 bits\n01 = 6 bits\n10 = 7 bits\n11 = 8 bits"]
    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UartLcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLcrReg {
    #[inline(always)]
    fn default() -> UartLcrReg {
        <crate::RegValueT<UartLcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLsrReg_SPEC;
impl crate::sealed::RegSpec for UartLsrReg_SPEC {
    type DataType = u16;
}

#[doc = "Line Status Register"]
pub type UartLsrReg = crate::RegValueT<UartLsrReg_SPEC>;

impl UartLsrReg {
    #[doc = "Transmitter Empty bit.\nThis bit is set whenever the Transmitter Holding Register and the Transmitter Shift Register are both empty."]
    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transmit Holding Register Empty bit.\nIf THRE mode is disabled (IER\\[7\\] set to zero), this bit indicates that the THR.\nThis bit is set whenever data is transferred from the THR to the transmitter shift register and no new data has been written to the THR. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled."]
    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Break Interrupt bit.\nThis is used to indicate the detection of a break sequence on the serial input data.\nIf in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic \'0\' state for longer than the sum of start time + data bits + parity + stop bits.\nIf in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic \'0\' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART.\nReading the LSR clears the BI bit. The BI indication occurs immediately and persists until the LSR is read."]
    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Framing Error bit.\nThis is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data.\nWhen a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no framing error\n1 = framing error\nReading the LSR clears the FE bit."]
    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Parity Error bit.\nThis is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set.\nIt should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no parity error\n1 = parity error\nReading the LSR clears the PE bit."]
    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Overrun error bit.\nThis is used to indicate the occurrence of an overrun error.\nThis occurs if a new data character was received before the previous data was read.\nThe OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten.\n0 = no overrun error\n1 = overrun error\nReading the LSR clears the OE bit."]
    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Data Ready bit.\nThis is used to indicate that the receiver contains at least one character in the RBR.\n0 = no data ready\n1 = data ready\nThis bit is cleared when the RBR is read."]
    #[inline(always)]
    pub fn uart_dr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLsrReg {
    #[inline(always)]
    fn default() -> UartLsrReg {
        <crate::RegValueT<UartLsrReg_SPEC> as RegisterValue<_>>::new(96)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartMcrReg_SPEC;
impl crate::sealed::RegSpec for UartMcrReg_SPEC {
    type DataType = u16;
}

#[doc = "Modem Control Register"]
pub type UartMcrReg = crate::RegValueT<UartMcrReg_SPEC>;

impl UartMcrReg {
    #[doc = "SIR Mode Enable.\nThis is used to enable/disable the IrDA SIR Mode features as described in \"IrDA 1.0 SIR Protocol\".\n0 = IrDA SIR Mode disabled\n1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    pub fn uart_sire(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LoopBack Bit.\nThis is used to put the UART into a diagnostic mode for test purposes.\nIf operating in UART mode (SIR_MODE not active, MCR\\[6\\] set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally.\nIf operating in infrared mode (SIR_MODE active, MCR\\[6\\] set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OUT2.\nThis is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is:\n0 = out2_n de-asserted (logic 1)\n1 = out2_n asserted (logic 0)\nNote that in Loopback mode (MCR\\[4\\] set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_out2(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "OUT1.\nThis is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is:\n0 = out1_n de-asserted (logic 1)\n1 = out1_n asserted (logic 0)\nNote that in Loopback mode (MCR\\[4\\] set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_out1(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartMcrReg {
    #[inline(always)]
    fn default() -> UartMcrReg {
        <crate::RegValueT<UartMcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRbrThrDllReg_SPEC;
impl crate::sealed::RegSpec for UartRbrThrDllReg_SPEC {
    type DataType = u16;
}

#[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
pub type UartRbrThrDllReg = crate::RegValueT<UartRbrThrDllReg_SPEC>;

impl UartRbrThrDllReg {
    #[doc = "Receive Buffer Register: (RBR).\nThis register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur.\nTransmit Holding Register: (THR)\nThis register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost.\nDivisor Latch (Low): (DLL)\nThis register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows:\nbaud rate = (serial clock freq) / (16 * divisor)\nNote that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data.\nFor the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartRbrThrDllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartRbrThrDllReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartRbrThrDllReg {
    #[inline(always)]
    fn default() -> UartRbrThrDllReg {
        <crate::RegValueT<UartRbrThrDllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSbcrReg_SPEC;
impl crate::sealed::RegSpec for UartSbcrReg_SPEC {
    type DataType = u16;
}

#[doc = "Shadow Break Control Register"]
pub type UartSbcrReg = crate::RegValueT<UartSbcrReg_SPEC>;

impl UartSbcrReg {
    #[doc = "Shadow Break Control Bit.\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device.\nIf set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared.\nIf SIR_MODE active (MCR\\[6\\] = 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    pub fn uart_shadow_break_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSbcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartSbcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSbcrReg {
    #[inline(always)]
    fn default() -> UartSbcrReg {
        <crate::RegValueT<UartSbcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartScrReg_SPEC;
impl crate::sealed::RegSpec for UartScrReg_SPEC {
    type DataType = u16;
}

#[doc = "Scratchpad Register"]
pub type UartScrReg = crate::RegValueT<UartScrReg_SPEC>;

impl UartScrReg {
    #[doc = "This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
    #[inline(always)]
    pub fn uart_scratch_pad(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartScrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UartScrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartScrReg {
    #[inline(always)]
    fn default() -> UartScrReg {
        <crate::RegValueT<UartScrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrrReg_SPEC;
impl crate::sealed::RegSpec for UartSrrReg_SPEC {
    type DataType = u16;
}

#[doc = "Software Reset Register."]
pub type UartSrrReg = crate::RegValueT<UartSrrReg_SPEC>;

impl UartSrrReg {
    #[doc = "UART Reset. This asynchronously resets the UART Ctrl and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    pub fn uart_ur(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartSrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSrrReg {
    #[inline(always)]
    fn default() -> UartSrrReg {
        <crate::RegValueT<UartSrrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUcvReg_SPEC;
impl crate::sealed::RegSpec for UartUcvReg_SPEC {
    type DataType = u32;
}

#[doc = "Component Version"]
pub type UartUcvReg = crate::RegValueT<UartUcvReg_SPEC>;

impl UartUcvReg {
    #[doc = "Component Version"]
    #[inline(always)]
    pub fn ucv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartUcvReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartUcvReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartUcvReg {
    #[inline(always)]
    fn default() -> UartUcvReg {
        <crate::RegValueT<UartUcvReg_SPEC> as RegisterValue<_>>::new(13610)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUsrReg_SPEC;
impl crate::sealed::RegSpec for UartUsrReg_SPEC {
    type DataType = u16;
}

#[doc = "UART Status register."]
pub type UartUsrReg = crate::RegValueT<UartUsrReg_SPEC>;

impl UartUsrReg {
    #[doc = "UART Busy. This indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 - DW_apb_uart is idle or inactive 1 - DW_apb_uart is busy (actively transferring data) Note that it is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in the THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled) the assertion of this bit will also be delayed by several cycles of the slower clock."]
    #[inline(always)]
    pub fn uart_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartUsrReg {
    #[inline(always)]
    fn default() -> UartUsrReg {
        <crate::RegValueT<UartUsrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
