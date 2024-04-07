#[doc = "Register `QDATA1BIG` reader"]
pub type R = crate::R<QDATA1BIGrs>;
#[doc = "Register `QDATA1BIG` writer"]
pub type W = crate::W<QDATA1BIGrs>;
#[doc = "Field `QDATA1BIG` reader - Quad Data 1 Big Endian Access"]
pub type Qdata1bigR = crate::FieldReader<u32>;
#[doc = "Field `QDATA1BIG` writer - Quad Data 1 Big Endian Access"]
pub type Qdata1bigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&self) -> Qdata1bigR {
        Qdata1bigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    #[must_use]
    pub fn qdata1big(&mut self) -> Qdata1bigW<QDATA1BIGrs> {
        Qdata1bigW::new(self, 0)
    }
}
#[doc = "QDATA1 Register Big Endian Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdata1big::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdata1big::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDATA1BIGrs;
impl crate::RegisterSpec for QDATA1BIGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdata1big::R`](R) reader structure"]
impl crate::Readable for QDATA1BIGrs {}
#[doc = "`write(|w| ..)` method takes [`qdata1big::W`](W) writer structure"]
impl crate::Writable for QDATA1BIGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDATA1BIG to value 0"]
impl crate::Resettable for QDATA1BIGrs {
    const RESET_VALUE: u32 = 0;
}
