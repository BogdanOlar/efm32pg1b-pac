///Register `COMP1` reader
pub type R = crate::R<COMP1rs>;
///Register `COMP1` writer
pub type W = crate::W<COMP1rs>;
///Field `COMP1` reader - Compare Value 1
pub type Comp1R = crate::FieldReader<u16>;
///Field `COMP1` writer - Compare Value 1
pub type Comp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Compare Value 1
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1")
            .field("comp1", &self.comp1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Compare Value 1
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<COMP1rs> {
        Comp1W::new(self, 0)
    }
}
///Compare Value Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`comp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COMP1rs;
impl crate::RegisterSpec for COMP1rs {
    type Ux = u32;
}
///`read()` method returns [`comp1::R`](R) reader structure
impl crate::Readable for COMP1rs {}
///`write(|w| ..)` method takes [`comp1::W`](W) writer structure
impl crate::Writable for COMP1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMP1 to value 0
impl crate::Resettable for COMP1rs {
    const RESET_VALUE: u32 = 0;
}
