///Register `DATA3` reader
pub type R = crate::R<DATA3rs>;
///Register `DATA3` writer
pub type W = crate::W<DATA3rs>;
///Field `DATA3` reader - Data 3 Access
pub type Data3R = crate::FieldReader<u32>;
///Field `DATA3` writer - Data 3 Access
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data 3 Access
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(self.bits)
    }
}
impl core::fmt::Debug for crate::generic::Reg<DATA3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Data 3 Access
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> Data3W<DATA3rs> {
        Data3W::new(self, 0)
    }
}
///DATA3 Register Access
///
///You can [`read`](crate::Reg::read) this register and get [`data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///<div class="warning">One or more dependent resources other than the current register are immediately affected by a read operation.</div>
pub struct DATA3rs;
impl crate::RegisterSpec for DATA3rs {
    type Ux = u32;
}
///`read()` method returns [`data3::R`](R) reader structure
impl crate::Readable for DATA3rs {}
///`write(|w| ..)` method takes [`data3::W`](W) writer structure
impl crate::Writable for DATA3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA3 to value 0
impl crate::Resettable for DATA3rs {
    const RESET_VALUE: u32 = 0;
}
