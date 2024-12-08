///Register `CH2_SRC` reader
pub type R = crate::R<CH2_SRCrs>;
///Register `CH2_SRC` writer
pub type W = crate::W<CH2_SRCrs>;
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
        f.debug_struct("CH2_SRC")
            .field("srcaddr", &self.srcaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Source Data Address
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SrcaddrW<CH2_SRCrs> {
        SrcaddrW::new(self, 0)
    }
}
///Channel Descriptor Source Data Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch2_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH2_SRCrs;
impl crate::RegisterSpec for CH2_SRCrs {
    type Ux = u32;
}
///`read()` method returns [`ch2_src::R`](R) reader structure
impl crate::Readable for CH2_SRCrs {}
///`write(|w| ..)` method takes [`ch2_src::W`](W) writer structure
impl crate::Writable for CH2_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH2_SRC to value 0
impl crate::Resettable for CH2_SRCrs {
    const RESET_VALUE: u32 = 0;
}
