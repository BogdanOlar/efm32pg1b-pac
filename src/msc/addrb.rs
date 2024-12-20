///Register `ADDRB` reader
pub type R = crate::R<ADDRBrs>;
///Register `ADDRB` writer
pub type W = crate::W<ADDRBrs>;
///Field `ADDRB` reader - Page Erase or Write Address Buffer
pub type AddrbR = crate::FieldReader<u32>;
///Field `ADDRB` writer - Page Erase or Write Address Buffer
pub type AddrbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Page Erase or Write Address Buffer
    #[inline(always)]
    pub fn addrb(&self) -> AddrbR {
        AddrbR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRB")
            .field("addrb", &self.addrb())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Page Erase or Write Address Buffer
    #[inline(always)]
    #[must_use]
    pub fn addrb(&mut self) -> AddrbW<ADDRBrs> {
        AddrbW::new(self, 0)
    }
}
///Page Erase/Write Address Buffer
///
///You can [`read`](crate::Reg::read) this register and get [`addrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADDRBrs;
impl crate::RegisterSpec for ADDRBrs {
    type Ux = u32;
}
///`read()` method returns [`addrb::R`](R) reader structure
impl crate::Readable for ADDRBrs {}
///`write(|w| ..)` method takes [`addrb::W`](W) writer structure
impl crate::Writable for ADDRBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADDRB to value 0
impl crate::Resettable for ADDRBrs {
    const RESET_VALUE: u32 = 0;
}
