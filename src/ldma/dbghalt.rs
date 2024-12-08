///Register `DBGHALT` reader
pub type R = crate::R<DBGHALTrs>;
///Register `DBGHALT` writer
pub type W = crate::W<DBGHALTrs>;
///Field `DBGHALT` reader - DMA Debug Halt
pub type DbghaltR = crate::FieldReader;
///Field `DBGHALT` writer - DMA Debug Halt
pub type DbghaltW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DMA Debug Halt
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGHALT")
            .field("dbghalt", &self.dbghalt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DMA Debug Halt
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DbghaltW<DBGHALTrs> {
        DbghaltW::new(self, 0)
    }
}
///DMA Channel Debug Halt Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbghalt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbghalt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGHALTrs;
impl crate::RegisterSpec for DBGHALTrs {
    type Ux = u32;
}
///`read()` method returns [`dbghalt::R`](R) reader structure
impl crate::Readable for DBGHALTrs {}
///`write(|w| ..)` method takes [`dbghalt::W`](W) writer structure
impl crate::Writable for DBGHALTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGHALT to value 0
impl crate::Resettable for DBGHALTrs {
    const RESET_VALUE: u32 = 0;
}
