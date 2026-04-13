///Register `SRC` reader
pub type R = crate::R<SRCrs>;
///Register `SRC` writer
pub type W = crate::W<SRCrs>;
///Field `SRCADDR` reader - Source Data Address
pub type SrcaddrR = crate::FieldReader<u32>;
///Field `SRCADDR` writer - Source Data Address
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Source Data Address
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC")
            .field("srcaddr", &self.srcaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Source Data Address
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<'_, SRCrs> {
        SrcaddrW::new(self, 0)
    }
}
///Channel Descriptor Source Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRCrs;
impl crate::RegisterSpec for SRCrs {
    type Ux = u32;
}
///`read()` method returns [`src::R`](R) reader structure
impl crate::Readable for SRCrs {}
///`write(|w| ..)` method takes [`src::W`](W) writer structure
impl crate::Writable for SRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRC to value 0
impl crate::Resettable for SRCrs {}
