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
#[doc = r"AES_HASH registers"]
unsafe impl ::core::marker::Send for super::AesHash {}
unsafe impl ::core::marker::Sync for super::AesHash {}
impl super::AesHash {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Crypto Clear interrupt request"]
    #[inline(always)]
    pub const fn crypto_clrirq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoClrirqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoClrirqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Crypto Control register"]
    #[inline(always)]
    pub const fn crypto_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Crypto DMA destination memory"]
    #[inline(always)]
    pub const fn crypto_dest_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoDestAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoDestAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Crypto DMA fetch register"]
    #[inline(always)]
    pub const fn crypto_fetch_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoFetchAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoFetchAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Crypto First position of the AES keys storage memory"]
    #[inline(always)]
    pub const fn crypto_keys_start(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoKeysStart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoKeysStart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Crypto Length of the input block in bytes"]
    #[inline(always)]
    pub const fn crypto_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoLenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoLenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Crypto Mode depended register 0"]
    #[inline(always)]
    pub const fn crypto_mreg0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Crypto Mode depended register 1"]
    #[inline(always)]
    pub const fn crypto_mreg1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Crypto Mode depended register 2"]
    #[inline(always)]
    pub const fn crypto_mreg2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Crypto Mode depended register 3"]
    #[inline(always)]
    pub const fn crypto_mreg3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoMreg3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoMreg3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Crypto Start calculation"]
    #[inline(always)]
    pub const fn crypto_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Crypto Status register"]
    #[inline(always)]
    pub const fn crypto_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CryptoStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CryptoStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoClrirqReg_SPEC;
impl crate::sealed::RegSpec for CryptoClrirqReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Clear interrupt request"]
pub type CryptoClrirqReg = crate::RegValueT<CryptoClrirqReg_SPEC>;

impl CryptoClrirqReg {
    #[doc = "Write 1 to clear a pending interrupt request."]
    #[inline(always)]
    pub fn crypto_clrirq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoClrirqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoClrirqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoClrirqReg {
    #[inline(always)]
    fn default() -> CryptoClrirqReg {
        <crate::RegValueT<CryptoClrirqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoCtrlReg_SPEC;
impl crate::sealed::RegSpec for CryptoCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Control register"]
pub type CryptoCtrlReg = crate::RegValueT<CryptoCtrlReg_SPEC>;

impl CryptoCtrlReg {
    #[doc = "It forces (active high) the execution of the key expansion process with the starting of the AES encryption/decryption process. The bit will be cleared automatically by the hardware, after the completion of the AES key expansion process."]
    #[inline(always)]
    pub fn crypto_aes_kexp(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 - Define that this is the last input block. When the current input is consumed by the crypto engine and the output data is written to the memory, the calculation ends (CRYPTO_INACTIVE goes to one).\n1 - The current input data block is not the last. More input data will follow. When the current input is consumed, the engine stops and waits for more data (CRYPTO_WAIT_FOR_IN goes to one)."]
    #[inline(always)]
    pub fn crypto_more_in(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of bytes minus one of the hash result which will be saved at the memory by the DMA. In relation with the selected hash algorithm the accepted values are:\nMD5: 0..15 -> 1-16 bytes\nSHA-1: 0..19 -> 1-20 bytes\nSHA-256: 0..31 -> 1 - 32 bytes\nSHA-256/224: 0..27 -> 1- 28 bytes\nSHA-384: 0..47 -> 1 - 48 bytes\nSHA-512: 0..63 -> 1 - 64 bytes\nSHA-512/224: 0..27 -> 1- 28 bytes\nSHA-512/256: 0..31 -> 1 - 32 bytes"]
    #[inline(always)]
    pub fn crypto_hash_out_len(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the type of the algorithm\n0 - The encryption algorithm (AES)\n1 - A hash algorithm.\nThe exact algorithm is defined by the fileds CRYPTO_ALG and CRYPTO_ALG_MD."]
    #[inline(always)]
    pub fn crypto_hash_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Request Enable\n0 - The interrupt generation ability is disabled.\n1 - The interrupt generation ability is enabled. Generates an interrupt request at the end of operation."]
    #[inline(always)]
    pub fn crypto_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Encryption/Decryption\n0 - Decryption\n1 - Encryption"]
    #[inline(always)]
    pub fn crypto_encdec(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The size of AES Key\n00 - 128 bits AES Key\n01 - 192 bits AES Key\n10 - 256 bits AES Key\n11 - 256 bits AES Key"]
    #[inline(always)]
    pub fn crypto_aes_key_sz(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Output Mode. This field makes sense only when the AES algorithm is selected (CRYPTO_HASH_SEL =0)\n0 - Write back to memory all the resulting data\n1 - Write back to memory only the final block of the resulting data"]
    #[inline(always)]
    pub fn crypto_out_md(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CryptoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "It defines the mode of operation of the AES algorithm when the controller is configured for an encryption/decryption processing (CRYPTO_HASH_SEL = 0).\n00 - ECB\n01 - ECB\n10 - CTR\n11 - CBC\n\nWhen the controller is configured to applies a HASH function, this field selects the desired HASH algorithm with the help of the CRYPTO_ALG.\n\n00 - HASH algorithms that are based on 32 bits operations\n01 - HASH algorithms that are based on 64 bits operations\n10 - Reserved\n11 - Reserved\n\nSee also the CRYPTO_ALG field."]
    #[inline(always)]
    pub fn crypto_alg_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Algorithm selection. When CRYPTO_HASH_SEL = 0 the only available choice is the AES algorithm.\n00 - AES\n01 - Reserved\n10 - Reserved\n11 - Reserved\n\nWhen CRYPTO_HASH_SEL = 1, this field selects the desired hash algorithms, with the help of the CRYPTO_ALG_MD field.\n\nIf CRYPTO_ALG_MD = 00\n00 - MD5\n01 - SHA-1\n10 - SHA-256/224\n11 - SHA-256\n\nIf CRYPTO_ALG_MD = 01\n00 - SHA-384\n01 - SHA-512\n10 - SHA-512/224\n11 - SHA-512/256"]
    #[inline(always)]
    pub fn crypto_alg(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, CryptoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,CryptoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoCtrlReg {
    #[inline(always)]
    fn default() -> CryptoCtrlReg {
        <crate::RegValueT<CryptoCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoDestAddrReg_SPEC;
impl crate::sealed::RegSpec for CryptoDestAddrReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto DMA destination memory"]
pub type CryptoDestAddrReg = crate::RegValueT<CryptoDestAddrReg_SPEC>;

impl CryptoDestAddrReg {
    #[doc = "Destination address at where the result of the processing is stored. The value of this register is updated as the calculation proceeds and the output data are written to the memory."]
    #[inline(always)]
    pub fn crypto_dest_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoDestAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoDestAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoDestAddrReg {
    #[inline(always)]
    fn default() -> CryptoDestAddrReg {
        <crate::RegValueT<CryptoDestAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoFetchAddrReg_SPEC;
impl crate::sealed::RegSpec for CryptoFetchAddrReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto DMA fetch register"]
pub type CryptoFetchAddrReg = crate::RegValueT<CryptoFetchAddrReg_SPEC>;

impl CryptoFetchAddrReg {
    #[doc = "The memory address from where will be retrieved the data that will be processed. The value of this register is updated as the calculation proceeds and the output data are written to the memory."]
    #[inline(always)]
    pub fn crypto_fetch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoFetchAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoFetchAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoFetchAddrReg {
    #[inline(always)]
    fn default() -> CryptoFetchAddrReg {
        <crate::RegValueT<CryptoFetchAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoKeysStart_SPEC;
impl crate::sealed::RegSpec for CryptoKeysStart_SPEC {
    type DataType = u32;
}

#[doc = "Crypto First position of the AES keys storage memory"]
pub type CryptoKeysStart = crate::RegValueT<CryptoKeysStart_SPEC>;

impl CryptoKeysStart {
    #[doc = "CRYPTO_KEY_(0-63)\nThis is the AES keys storage memory. This memory is accessible via AHB slave interface, only when the CRYPTO is inactive (CRYPTO_INACTIVE = 1)."]
    #[inline(always)]
    pub fn crypto_key_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoKeysStart_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoKeysStart_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoKeysStart {
    #[inline(always)]
    fn default() -> CryptoKeysStart {
        <crate::RegValueT<CryptoKeysStart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoLenReg_SPEC;
impl crate::sealed::RegSpec for CryptoLenReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Length of the input block in bytes"]
pub type CryptoLenReg = crate::RegValueT<CryptoLenReg_SPEC>;

impl CryptoLenReg {
    #[doc = "It contains the number of bytes of input data. If this number is not a multiple of a block size, the data is automatically extended with zeros. The value of this register is updated as the calculation proceeds and the output data are written to the memory."]
    #[inline(always)]
    pub fn crypto_len(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        CryptoLenReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            CryptoLenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoLenReg {
    #[inline(always)]
    fn default() -> CryptoLenReg {
        <crate::RegValueT<CryptoLenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg0Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg0Reg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Mode depended register 0"]
pub type CryptoMreg0Reg = crate::RegValueT<CryptoMreg0Reg_SPEC>;

impl CryptoMreg0Reg {
    #[doc = "It contains information that are depended by the mode of operation, when is used the AES algorithm:\nCBC - IV\\[31:0\\]\nCTR - CTRBLK\\[31:0\\]. It is the initial value of the 32 bits counter.\nAt any other mode, the contents of this register has no meaning."]
    #[inline(always)]
    pub fn crypto_mreg0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg0Reg {
    #[inline(always)]
    fn default() -> CryptoMreg0Reg {
        <crate::RegValueT<CryptoMreg0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg1Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Mode depended register 1"]
pub type CryptoMreg1Reg = crate::RegValueT<CryptoMreg1Reg_SPEC>;

impl CryptoMreg1Reg {
    #[doc = "It contains information that are depended by the mode of operation, when is used the AES algorithm:\nCBC - IV\\[63:32\\]\nCTR - CTRBLK\\[63:32\\]\nAt any other mode, the contents of this register has no meaning."]
    #[inline(always)]
    pub fn crypto_mreg1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg1Reg {
    #[inline(always)]
    fn default() -> CryptoMreg1Reg {
        <crate::RegValueT<CryptoMreg1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg2Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Mode depended register 2"]
pub type CryptoMreg2Reg = crate::RegValueT<CryptoMreg2Reg_SPEC>;

impl CryptoMreg2Reg {
    #[doc = "It contains information that are depended by the mode of operation, when is used the AES algorithm:\nCBC - IV\\[95:64\\]\nCTR - CTRBLK\\[95:64\\]\nAt any other mode, the contents of this register has no meaning."]
    #[inline(always)]
    pub fn crypto_mreg2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg2Reg {
    #[inline(always)]
    fn default() -> CryptoMreg2Reg {
        <crate::RegValueT<CryptoMreg2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoMreg3Reg_SPEC;
impl crate::sealed::RegSpec for CryptoMreg3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Mode depended register 3"]
pub type CryptoMreg3Reg = crate::RegValueT<CryptoMreg3Reg_SPEC>;

impl CryptoMreg3Reg {
    #[doc = "It contains information that are depended by the mode of operation, when is used the AES algorithm:\nCBC - IV\\[127:96\\]\nCTR - CTRBLK\\[127:96\\]\nAt any other mode, the contents of this register has no meaning."]
    #[inline(always)]
    pub fn crypto_mreg3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CryptoMreg3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CryptoMreg3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CryptoMreg3Reg {
    #[inline(always)]
    fn default() -> CryptoMreg3Reg {
        <crate::RegValueT<CryptoMreg3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoStartReg_SPEC;
impl crate::sealed::RegSpec for CryptoStartReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Start calculation"]
pub type CryptoStartReg = crate::RegValueT<CryptoStartReg_SPEC>;

impl CryptoStartReg {
    #[doc = "Write 1 to initiate the processing of the input data. This register is auto-cleared."]
    #[inline(always)]
    pub fn crypto_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoStartReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoStartReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoStartReg {
    #[inline(always)]
    fn default() -> CryptoStartReg {
        <crate::RegValueT<CryptoStartReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoStatusReg_SPEC;
impl crate::sealed::RegSpec for CryptoStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Crypto Status register"]
pub type CryptoStatusReg = crate::RegValueT<CryptoStatusReg_SPEC>;

impl CryptoStatusReg {
    #[doc = "The status of the interrupt request line of the CRYPTO block.\n0 - There is no active interrupt request.\n1 - An interrupt request is pending."]
    #[inline(always)]
    pub fn crypto_irq_st(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the situation where the engine waits for more input data. This is applicable when the CRYPTO_MORE_IN= 1, so the input data are fragmented in the memory.\n0 - The crypto is not waiting for more input data.\n1 - The crypto waits for more input data.\nThe CRYPTO_INACTIVE flag remains to zero to indicate that the calculation is not finished. The supervisor of the CRYPTO must program to the CRYPTO_FETCH_ADDR and CRYPTO_LEN a new input data fragment. The calculation will be continued as soon as the CRYPTO_START register will be written with 1. This action will clear the CRYPTO_WAIT_FOR_IN flag."]
    #[inline(always)]
    pub fn crypto_wait_for_in(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 - The CRYPTO is active. The processing is in progress.\n1 - The CRYPTO is inactive. The processing has finished."]
    #[inline(always)]
    pub fn crypto_inactive(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CryptoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CryptoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CryptoStatusReg {
    #[inline(always)]
    fn default() -> CryptoStatusReg {
        <crate::RegValueT<CryptoStatusReg_SPEC> as RegisterValue<_>>::new(1)
    }
}
