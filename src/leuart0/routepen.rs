///Register `ROUTEPEN` reader
pub type R = crate::R<ROUTEPENrs>;
///Register `ROUTEPEN` writer
pub type W = crate::W<ROUTEPENrs>;
///Field `RXPEN` reader - RX Pin Enable
pub type RxpenR = crate::BitReader;
///Field `RXPEN` writer - RX Pin Enable
pub type RxpenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPEN` reader - TX Pin Enable
pub type TxpenR = crate::BitReader;
///Field `TXPEN` writer - TX Pin Enable
pub type TxpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RX Pin Enable
    #[inline(always)]
    pub fn rxpen(&self) -> RxpenR {
        RxpenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Pin Enable
    #[inline(always)]
    pub fn txpen(&self) -> TxpenR {
        TxpenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("rxpen", &self.rxpen())
            .field("txpen", &self.txpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RX Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn rxpen(&mut self) -> RxpenW<ROUTEPENrs> {
        RxpenW::new(self, 0)
    }
    ///Bit 1 - TX Pin Enable
    #[inline(always)]
    #[must_use]
    pub fn txpen(&mut self) -> TxpenW<ROUTEPENrs> {
        TxpenW::new(self, 1)
    }
}
///I/O Routing Pin Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
///`read()` method returns [`routepen::R`](R) reader structure
impl crate::Readable for ROUTEPENrs {}
///`write(|w| ..)` method takes [`routepen::W`](W) writer structure
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTEPEN to value 0
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
