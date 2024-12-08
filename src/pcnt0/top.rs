///Register `TOP` reader
pub type R = crate::R<TOPrs>;
///Field `TOP` reader - Counter Top Value
pub type TopR = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Counter Top Value
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOP").field("top", &self.top()).finish()
    }
}
///Top Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`top::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TOPrs;
impl crate::RegisterSpec for TOPrs {
    type Ux = u32;
}
///`read()` method returns [`top::R`](R) reader structure
impl crate::Readable for TOPrs {}
///`reset()` method sets TOP to value 0xff
impl crate::Resettable for TOPrs {
    const RESET_VALUE: u32 = 0xff;
}
