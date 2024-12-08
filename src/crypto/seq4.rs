///Register `SEQ4` reader
pub type R = crate::R<SEQ4rs>;
///Register `SEQ4` writer
pub type W = crate::W<SEQ4rs>;
///Field `INSTR16` reader - Sequence Instruction 16
pub type Instr16R = crate::FieldReader;
///Field `INSTR16` writer - Sequence Instruction 16
pub type Instr16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR17` reader - Sequence Instruction 17
pub type Instr17R = crate::FieldReader;
///Field `INSTR17` writer - Sequence Instruction 17
pub type Instr17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR18` reader - Sequence Instruction 18
pub type Instr18R = crate::FieldReader;
///Field `INSTR18` writer - Sequence Instruction 18
pub type Instr18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR19` reader - Sequence Instruction 19
pub type Instr19R = crate::FieldReader;
///Field `INSTR19` writer - Sequence Instruction 19
pub type Instr19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sequence Instruction 16
    #[inline(always)]
    pub fn instr16(&self) -> Instr16R {
        Instr16R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sequence Instruction 17
    #[inline(always)]
    pub fn instr17(&self) -> Instr17R {
        Instr17R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Sequence Instruction 18
    #[inline(always)]
    pub fn instr18(&self) -> Instr18R {
        Instr18R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Sequence Instruction 19
    #[inline(always)]
    pub fn instr19(&self) -> Instr19R {
        Instr19R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ4")
            .field("instr16", &self.instr16())
            .field("instr17", &self.instr17())
            .field("instr18", &self.instr18())
            .field("instr19", &self.instr19())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Sequence Instruction 16
    #[inline(always)]
    #[must_use]
    pub fn instr16(&mut self) -> Instr16W<SEQ4rs> {
        Instr16W::new(self, 0)
    }
    ///Bits 8:15 - Sequence Instruction 17
    #[inline(always)]
    #[must_use]
    pub fn instr17(&mut self) -> Instr17W<SEQ4rs> {
        Instr17W::new(self, 8)
    }
    ///Bits 16:23 - Sequence Instruction 18
    #[inline(always)]
    #[must_use]
    pub fn instr18(&mut self) -> Instr18W<SEQ4rs> {
        Instr18W::new(self, 16)
    }
    ///Bits 24:31 - Sequence Instruction 19
    #[inline(always)]
    #[must_use]
    pub fn instr19(&mut self) -> Instr19W<SEQ4rs> {
        Instr19W::new(self, 24)
    }
}
///Sequence Register 4
///
///You can [`read`](crate::Reg::read) this register and get [`seq4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQ4rs;
impl crate::RegisterSpec for SEQ4rs {
    type Ux = u32;
}
///`read()` method returns [`seq4::R`](R) reader structure
impl crate::Readable for SEQ4rs {}
///`write(|w| ..)` method takes [`seq4::W`](W) writer structure
impl crate::Writable for SEQ4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ4 to value 0
impl crate::Resettable for SEQ4rs {
    const RESET_VALUE: u32 = 0;
}
