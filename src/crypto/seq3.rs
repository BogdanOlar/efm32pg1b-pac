///Register `SEQ3` reader
pub type R = crate::R<SEQ3rs>;
///Register `SEQ3` writer
pub type W = crate::W<SEQ3rs>;
///Field `INSTR12` reader - Sequence Instruction 12
pub type Instr12R = crate::FieldReader;
///Field `INSTR12` writer - Sequence Instruction 12
pub type Instr12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR13` reader - Sequence Instruction 13
pub type Instr13R = crate::FieldReader;
///Field `INSTR13` writer - Sequence Instruction 13
pub type Instr13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR14` reader - Sequence Instruction 14
pub type Instr14R = crate::FieldReader;
///Field `INSTR14` writer - Sequence Instruction 14
pub type Instr14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR15` reader - Sequence Instruction 15
pub type Instr15R = crate::FieldReader;
///Field `INSTR15` writer - Sequence Instruction 15
pub type Instr15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sequence Instruction 12
    #[inline(always)]
    pub fn instr12(&self) -> Instr12R {
        Instr12R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sequence Instruction 13
    #[inline(always)]
    pub fn instr13(&self) -> Instr13R {
        Instr13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Sequence Instruction 14
    #[inline(always)]
    pub fn instr14(&self) -> Instr14R {
        Instr14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Sequence Instruction 15
    #[inline(always)]
    pub fn instr15(&self) -> Instr15R {
        Instr15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ3")
            .field("instr12", &self.instr12())
            .field("instr13", &self.instr13())
            .field("instr14", &self.instr14())
            .field("instr15", &self.instr15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Sequence Instruction 12
    #[inline(always)]
    #[must_use]
    pub fn instr12(&mut self) -> Instr12W<SEQ3rs> {
        Instr12W::new(self, 0)
    }
    ///Bits 8:15 - Sequence Instruction 13
    #[inline(always)]
    #[must_use]
    pub fn instr13(&mut self) -> Instr13W<SEQ3rs> {
        Instr13W::new(self, 8)
    }
    ///Bits 16:23 - Sequence Instruction 14
    #[inline(always)]
    #[must_use]
    pub fn instr14(&mut self) -> Instr14W<SEQ3rs> {
        Instr14W::new(self, 16)
    }
    ///Bits 24:31 - Sequence Instruction 15
    #[inline(always)]
    #[must_use]
    pub fn instr15(&mut self) -> Instr15W<SEQ3rs> {
        Instr15W::new(self, 24)
    }
}
///Sequence Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`seq3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQ3rs;
impl crate::RegisterSpec for SEQ3rs {
    type Ux = u32;
}
///`read()` method returns [`seq3::R`](R) reader structure
impl crate::Readable for SEQ3rs {}
///`write(|w| ..)` method takes [`seq3::W`](W) writer structure
impl crate::Writable for SEQ3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ3 to value 0
impl crate::Resettable for SEQ3rs {
    const RESET_VALUE: u32 = 0;
}
