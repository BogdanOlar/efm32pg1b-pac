///Register `SEQ1` reader
pub type R = crate::R<SEQ1rs>;
///Register `SEQ1` writer
pub type W = crate::W<SEQ1rs>;
///Field `INSTR4` reader - Sequence Instruction 4
pub type Instr4R = crate::FieldReader;
///Field `INSTR4` writer - Sequence Instruction 4
pub type Instr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR5` reader - Sequence Instruction 5
pub type Instr5R = crate::FieldReader;
///Field `INSTR5` writer - Sequence Instruction 5
pub type Instr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR6` reader - Sequence Instruction 6
pub type Instr6R = crate::FieldReader;
///Field `INSTR6` writer - Sequence Instruction 6
pub type Instr6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `INSTR7` reader - Sequence Instruction 7
pub type Instr7R = crate::FieldReader;
///Field `INSTR7` writer - Sequence Instruction 7
pub type Instr7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sequence Instruction 4
    #[inline(always)]
    pub fn instr4(&self) -> Instr4R {
        Instr4R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sequence Instruction 5
    #[inline(always)]
    pub fn instr5(&self) -> Instr5R {
        Instr5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Sequence Instruction 6
    #[inline(always)]
    pub fn instr6(&self) -> Instr6R {
        Instr6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Sequence Instruction 7
    #[inline(always)]
    pub fn instr7(&self) -> Instr7R {
        Instr7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ1")
            .field("instr4", &self.instr4())
            .field("instr5", &self.instr5())
            .field("instr6", &self.instr6())
            .field("instr7", &self.instr7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Sequence Instruction 4
    #[inline(always)]
    #[must_use]
    pub fn instr4(&mut self) -> Instr4W<SEQ1rs> {
        Instr4W::new(self, 0)
    }
    ///Bits 8:15 - Sequence Instruction 5
    #[inline(always)]
    #[must_use]
    pub fn instr5(&mut self) -> Instr5W<SEQ1rs> {
        Instr5W::new(self, 8)
    }
    ///Bits 16:23 - Sequence Instruction 6
    #[inline(always)]
    #[must_use]
    pub fn instr6(&mut self) -> Instr6W<SEQ1rs> {
        Instr6W::new(self, 16)
    }
    ///Bits 24:31 - Sequence Instruction 7
    #[inline(always)]
    #[must_use]
    pub fn instr7(&mut self) -> Instr7W<SEQ1rs> {
        Instr7W::new(self, 24)
    }
}
///Sequence Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`seq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SEQ1rs;
impl crate::RegisterSpec for SEQ1rs {
    type Ux = u32;
}
///`read()` method returns [`seq1::R`](R) reader structure
impl crate::Readable for SEQ1rs {}
///`write(|w| ..)` method takes [`seq1::W`](W) writer structure
impl crate::Writable for SEQ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEQ1 to value 0
impl crate::Resettable for SEQ1rs {
    const RESET_VALUE: u32 = 0;
}
