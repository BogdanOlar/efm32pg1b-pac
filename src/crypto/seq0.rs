///Register `SEQ0` reader
pub type R = crate::R<SEQ0rs>;
///Register `SEQ0` writer
pub type W = crate::W<SEQ0rs>;
///Field `INSTR0` reader - Sequence Instruction 0
pub type Instr0R = crate::FieldReader;
///Field `INSTR0` writer - Sequence Instruction 0
pub type Instr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR1` reader - Sequence Instruction 1
pub type Instr1R = crate::FieldReader;
///Field `INSTR1` writer - Sequence Instruction 1
pub type Instr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR2` reader - Sequence Instruction 2
pub type Instr2R = crate::FieldReader;
///Field `INSTR2` writer - Sequence Instruction 2
pub type Instr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR3` reader - Sequence Instruction 3
pub type Instr3R = crate::FieldReader;
///Field `INSTR3` writer - Sequence Instruction 3
pub type Instr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sequence Instruction 0
    #[inline(always)]
    pub fn instr0(&self) -> Instr0R {
        Instr0R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sequence Instruction 1
    #[inline(always)]
    pub fn instr1(&self) -> Instr1R {
        Instr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Sequence Instruction 2
    #[inline(always)]
    pub fn instr2(&self) -> Instr2R {
        Instr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Sequence Instruction 3
    #[inline(always)]
    pub fn instr3(&self) -> Instr3R {
        Instr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ0")
            .field("instr0", &self.instr0())
            .field("instr1", &self.instr1())
            .field("instr2", &self.instr2())
            .field("instr3", &self.instr3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Sequence Instruction 0
    #[inline(always)]
    #[must_use]
    pub fn instr0(&mut self) -> Instr0W<SEQ0rs> {
        Instr0W::new(self, 0)
    }
    ///Bits 8:15 - Sequence Instruction 1
    #[inline(always)]
    #[must_use]
    pub fn instr1(&mut self) -> Instr1W<SEQ0rs> {
        Instr1W::new(self, 8)
    }
    ///Bits 16:23 - Sequence Instruction 2
    #[inline(always)]
    #[must_use]
    pub fn instr2(&mut self) -> Instr2W<SEQ0rs> {
        Instr2W::new(self, 16)
    }
    ///Bits 24:31 - Sequence Instruction 3
    #[inline(always)]
    #[must_use]
    pub fn instr3(&mut self) -> Instr3W<SEQ0rs> {
        Instr3W::new(self, 24)
    }
}
///Sequence Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`seq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQ0rs;
impl crate::RegisterSpec for SEQ0rs {
    type Ux = u32;
}
///`read()` method returns [`seq0::R`](R) reader structure
impl crate::Readable for SEQ0rs {}
///`write(|w| ..)` method takes [`seq0::W`](W) writer structure
impl crate::Writable for SEQ0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ0 to value 0
impl crate::Resettable for SEQ0rs {
    const RESET_VALUE: u32 = 0;
}
