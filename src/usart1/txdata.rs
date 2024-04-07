#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TXDATArs>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TXDATArs>;
#[doc = "Field `TXDATA` reader - TX Data"]
pub type TxdataR = crate::FieldReader;
#[doc = "Field `TXDATA` writer - TX Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TXDATArs> {
        TxdataW::new(self, 0)
    }
}
#[doc = "TX Buffer Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATArs;
impl crate::RegisterSpec for TXDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TXDATArs {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TXDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TXDATArs {
    const RESET_VALUE: u32 = 0;
}
