///Register `SADDR` reader
pub type R = crate::R<SADDRrs>;
///Register `SADDR` writer
pub type W = crate::W<SADDRrs>;
///Field `ADDR` reader - Slave Address
pub type AddrR = crate::FieldReader;
///Field `ADDR` writer - Slave Address
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 1:7 - Slave Address
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADDR").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 1:7 - Slave Address
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SADDRrs> {
        AddrW::new(self, 1)
    }
}
///Slave Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SADDRrs;
impl crate::RegisterSpec for SADDRrs {
    type Ux = u32;
}
///`read()` method returns [`saddr::R`](R) reader structure
impl crate::Readable for SADDRrs {}
///`write(|w| ..)` method takes [`saddr::W`](W) writer structure
impl crate::Writable for SADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SADDR to value 0
impl crate::Resettable for SADDRrs {
    const RESET_VALUE: u32 = 0;
}
