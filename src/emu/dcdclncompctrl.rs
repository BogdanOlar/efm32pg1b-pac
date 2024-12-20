///Register `DCDCLNCOMPCTRL` reader
pub type R = crate::R<DCDCLNCOMPCTRLrs>;
///Register `DCDCLNCOMPCTRL` writer
pub type W = crate::W<DCDCLNCOMPCTRLrs>;
///Field `COMPENR1` reader - Low Noise Mode Compensator R1 Trim Value
pub type Compenr1R = crate::FieldReader;
///Field `COMPENR1` writer - Low Noise Mode Compensator R1 Trim Value
pub type Compenr1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMPENR2` reader - Low Noise Mode Compensator R2 Trim Value
pub type Compenr2R = crate::FieldReader;
///Field `COMPENR2` writer - Low Noise Mode Compensator R2 Trim Value
pub type Compenr2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `COMPENR3` reader - Low Noise Mode Compensator R3 Trim Value
pub type Compenr3R = crate::FieldReader;
///Field `COMPENR3` writer - Low Noise Mode Compensator R3 Trim Value
pub type Compenr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `COMPENC1` reader - Low Noise Mode Compensator C1 Trim Value
pub type Compenc1R = crate::FieldReader;
///Field `COMPENC1` writer - Low Noise Mode Compensator C1 Trim Value
pub type Compenc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMPENC2` reader - Low Noise Mode Compensator C2 Trim Value
pub type Compenc2R = crate::FieldReader;
///Field `COMPENC2` writer - Low Noise Mode Compensator C2 Trim Value
pub type Compenc2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `COMPENC3` reader - Low Noise Mode Compensator C3 Trim Value
pub type Compenc3R = crate::FieldReader;
///Field `COMPENC3` writer - Low Noise Mode Compensator C3 Trim Value
pub type Compenc3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - Low Noise Mode Compensator R1 Trim Value
    #[inline(always)]
    pub fn compenr1(&self) -> Compenr1R {
        Compenr1R::new((self.bits & 7) as u8)
    }
    ///Bits 4:8 - Low Noise Mode Compensator R2 Trim Value
    #[inline(always)]
    pub fn compenr2(&self) -> Compenr2R {
        Compenr2R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    ///Bits 12:15 - Low Noise Mode Compensator R3 Trim Value
    #[inline(always)]
    pub fn compenr3(&self) -> Compenr3R {
        Compenr3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:21 - Low Noise Mode Compensator C1 Trim Value
    #[inline(always)]
    pub fn compenc1(&self) -> Compenc1R {
        Compenc1R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:26 - Low Noise Mode Compensator C2 Trim Value
    #[inline(always)]
    pub fn compenc2(&self) -> Compenc2R {
        Compenc2R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:31 - Low Noise Mode Compensator C3 Trim Value
    #[inline(always)]
    pub fn compenc3(&self) -> Compenc3R {
        Compenc3R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCLNCOMPCTRL")
            .field("compenr1", &self.compenr1())
            .field("compenr2", &self.compenr2())
            .field("compenr3", &self.compenr3())
            .field("compenc1", &self.compenc1())
            .field("compenc2", &self.compenc2())
            .field("compenc3", &self.compenc3())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low Noise Mode Compensator R1 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenr1(&mut self) -> Compenr1W<DCDCLNCOMPCTRLrs> {
        Compenr1W::new(self, 0)
    }
    ///Bits 4:8 - Low Noise Mode Compensator R2 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenr2(&mut self) -> Compenr2W<DCDCLNCOMPCTRLrs> {
        Compenr2W::new(self, 4)
    }
    ///Bits 12:15 - Low Noise Mode Compensator R3 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenr3(&mut self) -> Compenr3W<DCDCLNCOMPCTRLrs> {
        Compenr3W::new(self, 12)
    }
    ///Bits 20:21 - Low Noise Mode Compensator C1 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenc1(&mut self) -> Compenc1W<DCDCLNCOMPCTRLrs> {
        Compenc1W::new(self, 20)
    }
    ///Bits 24:26 - Low Noise Mode Compensator C2 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenc2(&mut self) -> Compenc2W<DCDCLNCOMPCTRLrs> {
        Compenc2W::new(self, 24)
    }
    ///Bits 28:31 - Low Noise Mode Compensator C3 Trim Value
    #[inline(always)]
    #[must_use]
    pub fn compenc3(&mut self) -> Compenc3W<DCDCLNCOMPCTRLrs> {
        Compenc3W::new(self, 28)
    }
}
///DCDC Low Noise Compensator Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdclncompctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclncompctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCLNCOMPCTRLrs;
impl crate::RegisterSpec for DCDCLNCOMPCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dcdclncompctrl::R`](R) reader structure
impl crate::Readable for DCDCLNCOMPCTRLrs {}
///`write(|w| ..)` method takes [`dcdclncompctrl::W`](W) writer structure
impl crate::Writable for DCDCLNCOMPCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCLNCOMPCTRL to value 0x5720_4077
impl crate::Resettable for DCDCLNCOMPCTRLrs {
    const RESET_VALUE: u32 = 0x5720_4077;
}
