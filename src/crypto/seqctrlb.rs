///Register `SEQCTRLB` reader
pub type R = crate::R<SEQCTRLBrs>;
///Register `SEQCTRLB` writer
pub type W = crate::W<SEQCTRLBrs>;
///Field `LENGTHB` reader - Buffer Length B in Bytes
pub type LengthbR = crate::FieldReader<u16>;
///Field `LENGTHB` writer - Buffer Length B in Bytes
pub type LengthbW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `DMA0PRESB` reader - DMA0 Preserve B
pub type Dma0presbR = crate::BitReader;
///Field `DMA0PRESB` writer - DMA0 Preserve B
pub type Dma0presbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA1PRESB` reader - DMA1 Preserve B
pub type Dma1presbR = crate::BitReader;
///Field `DMA1PRESB` writer - DMA1 Preserve B
pub type Dma1presbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - Buffer Length B in Bytes
    #[inline(always)]
    pub fn lengthb(&self) -> LengthbR {
        LengthbR::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 28 - DMA0 Preserve B
    #[inline(always)]
    pub fn dma0presb(&self) -> Dma0presbR {
        Dma0presbR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DMA1 Preserve B
    #[inline(always)]
    pub fn dma1presb(&self) -> Dma1presbR {
        Dma1presbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQCTRLB")
            .field("lengthb", &self.lengthb())
            .field("dma0presb", &self.dma0presb())
            .field("dma1presb", &self.dma1presb())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Buffer Length B in Bytes
    #[inline(always)]
    #[must_use]
    pub fn lengthb(&mut self) -> LengthbW<SEQCTRLBrs> {
        LengthbW::new(self, 0)
    }
    ///Bit 28 - DMA0 Preserve B
    #[inline(always)]
    #[must_use]
    pub fn dma0presb(&mut self) -> Dma0presbW<SEQCTRLBrs> {
        Dma0presbW::new(self, 28)
    }
    ///Bit 29 - DMA1 Preserve B
    #[inline(always)]
    #[must_use]
    pub fn dma1presb(&mut self) -> Dma1presbW<SEQCTRLBrs> {
        Dma1presbW::new(self, 29)
    }
}
///Sequence Control B
///
///You can [`read`](crate::Reg::read) this register and get [`seqctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQCTRLBrs;
impl crate::RegisterSpec for SEQCTRLBrs {
    type Ux = u32;
}
///`read()` method returns [`seqctrlb::R`](R) reader structure
impl crate::Readable for SEQCTRLBrs {}
///`write(|w| ..)` method takes [`seqctrlb::W`](W) writer structure
impl crate::Writable for SEQCTRLBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQCTRLB to value 0
impl crate::Resettable for SEQCTRLBrs {
    const RESET_VALUE: u32 = 0;
}
