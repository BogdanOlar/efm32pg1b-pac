///Register `TXDATA` reader
pub type R = crate::R<TXDATArs>;
///Register `TXDATA` writer
pub type W = crate::W<TXDATArs>;
///Field `TXDATA` reader - TX Data
pub type TxdataR = crate::FieldReader;
///Field `TXDATA` writer - TX Data
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - TX Data
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDATA")
            .field("txdata", &self.txdata())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - TX Data
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<TXDATArs> {
        TxdataW::new(self, 0)
    }
}
///TX Buffer Data Register
///
///You can [`read`](crate::Reg::read) this register and get [`txdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TXDATArs;
impl crate::RegisterSpec for TXDATArs {
    type Ux = u32;
}
///`read()` method returns [`txdata::R`](R) reader structure
impl crate::Readable for TXDATArs {}
///`write(|w| ..)` method takes [`txdata::W`](W) writer structure
impl crate::Writable for TXDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXDATA to value 0
impl crate::Resettable for TXDATArs {
    const RESET_VALUE: u32 = 0;
}
