///Register `SEQCTRL` reader
pub type R = crate::R<SEQCTRLrs>;
///Register `SEQCTRL` writer
pub type W = crate::W<SEQCTRLrs>;
///Field `LENGTHA` reader - Buffer Length a in Bytes
pub type LengthaR = crate::FieldReader<u16>;
///Field `LENGTHA` writer - Buffer Length a in Bytes
pub type LengthaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Size of Data Blocks
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCKSIZE {
    ///0: A block is 16 bytes long
    _16bytes = 0,
    ///1: A block is 32 bytes long
    _32bytes = 1,
    ///2: A block is 64 bytes long
    _64bytes = 2,
}
impl From<BLOCKSIZE> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLOCKSIZE {
    type Ux = u8;
}
impl crate::IsEnum for BLOCKSIZE {}
///Field `BLOCKSIZE` reader - Size of Data Blocks
pub type BlocksizeR = crate::FieldReader<BLOCKSIZE>;
impl BlocksizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLOCKSIZE> {
        match self.bits {
            0 => Some(BLOCKSIZE::_16bytes),
            1 => Some(BLOCKSIZE::_32bytes),
            2 => Some(BLOCKSIZE::_64bytes),
            _ => None,
        }
    }
    ///A block is 16 bytes long
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == BLOCKSIZE::_16bytes
    }
    ///A block is 32 bytes long
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == BLOCKSIZE::_32bytes
    }
    ///A block is 64 bytes long
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == BLOCKSIZE::_64bytes
    }
}
///Field `BLOCKSIZE` writer - Size of Data Blocks
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BLOCKSIZE>;
impl<'a, REG> BlocksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///A block is 16 bytes long
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::_16bytes)
    }
    ///A block is 32 bytes long
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::_32bytes)
    }
    ///A block is 64 bytes long
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::_64bytes)
    }
}
///Field `DMA0SKIP` reader - DMA0 Skip
pub type Dma0skipR = crate::FieldReader;
///Field `DMA0SKIP` writer - DMA0 Skip
pub type Dma0skipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMA1SKIP` reader - DMA1 Skip
pub type Dma1skipR = crate::FieldReader;
///Field `DMA1SKIP` writer - DMA1 Skip
pub type Dma1skipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DMA0PRESA` reader - DMA0 Preserve a
pub type Dma0presaR = crate::BitReader;
///Field `DMA0PRESA` writer - DMA0 Preserve a
pub type Dma0presaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1PRESA` reader - DMA1 Preserve a
pub type Dma1presaR = crate::BitReader;
///Field `DMA1PRESA` writer - DMA1 Preserve a
pub type Dma1presaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HALT` reader - Halt Sequence
pub type HaltR = crate::BitReader;
///Field `HALT` writer - Halt Sequence
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - Buffer Length a in Bytes
    #[inline(always)]
    pub fn lengtha(&self) -> LengthaR {
        LengthaR::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 20:21 - Size of Data Blocks
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - DMA0 Skip
    #[inline(always)]
    pub fn dma0skip(&self) -> Dma0skipR {
        Dma0skipR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - DMA1 Skip
    #[inline(always)]
    pub fn dma1skip(&self) -> Dma1skipR {
        Dma1skipR::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - DMA0 Preserve a
    #[inline(always)]
    pub fn dma0presa(&self) -> Dma0presaR {
        Dma0presaR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DMA1 Preserve a
    #[inline(always)]
    pub fn dma1presa(&self) -> Dma1presaR {
        Dma1presaR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Halt Sequence
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQCTRL")
            .field("lengtha", &self.lengtha())
            .field("blocksize", &self.blocksize())
            .field("dma0skip", &self.dma0skip())
            .field("dma1skip", &self.dma1skip())
            .field("dma0presa", &self.dma0presa())
            .field("dma1presa", &self.dma1presa())
            .field("halt", &self.halt())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Buffer Length a in Bytes
    #[inline(always)]
    #[must_use]
    pub fn lengtha(&mut self) -> LengthaW<SEQCTRLrs> {
        LengthaW::new(self, 0)
    }
    ///Bits 20:21 - Size of Data Blocks
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BlocksizeW<SEQCTRLrs> {
        BlocksizeW::new(self, 20)
    }
    ///Bits 24:25 - DMA0 Skip
    #[inline(always)]
    #[must_use]
    pub fn dma0skip(&mut self) -> Dma0skipW<SEQCTRLrs> {
        Dma0skipW::new(self, 24)
    }
    ///Bits 26:27 - DMA1 Skip
    #[inline(always)]
    #[must_use]
    pub fn dma1skip(&mut self) -> Dma1skipW<SEQCTRLrs> {
        Dma1skipW::new(self, 26)
    }
    ///Bit 28 - DMA0 Preserve a
    #[inline(always)]
    #[must_use]
    pub fn dma0presa(&mut self) -> Dma0presaW<SEQCTRLrs> {
        Dma0presaW::new(self, 28)
    }
    ///Bit 29 - DMA1 Preserve a
    #[inline(always)]
    #[must_use]
    pub fn dma1presa(&mut self) -> Dma1presaW<SEQCTRLrs> {
        Dma1presaW::new(self, 29)
    }
    ///Bit 31 - Halt Sequence
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<SEQCTRLrs> {
        HaltW::new(self, 31)
    }
}
///Sequence Control
///
///You can [`read`](crate::Reg::read) this register and get [`seqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQCTRLrs;
impl crate::RegisterSpec for SEQCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`seqctrl::R`](R) reader structure
impl crate::Readable for SEQCTRLrs {}
///`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure
impl crate::Writable for SEQCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQCTRL to value 0
impl crate::Resettable for SEQCTRLrs {
    const RESET_VALUE: u32 = 0;
}
