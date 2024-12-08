///Register `DTOGEN` reader
pub type R = crate::R<DTOGENrs>;
///Register `DTOGEN` writer
pub type W = crate::W<DTOGENrs>;
///Field `DTOGCC0EN` reader - DTI CC0 Output Generation Enable
pub type Dtogcc0enR = crate::BitReader;
///Field `DTOGCC0EN` writer - DTI CC0 Output Generation Enable
pub type Dtogcc0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTOGCC1EN` reader - DTI CC1 Output Generation Enable
pub type Dtogcc1enR = crate::BitReader;
///Field `DTOGCC1EN` writer - DTI CC1 Output Generation Enable
pub type Dtogcc1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTOGCC2EN` reader - DTI CC2 Output Generation Enable
pub type Dtogcc2enR = crate::BitReader;
///Field `DTOGCC2EN` writer - DTI CC2 Output Generation Enable
pub type Dtogcc2enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTOGCDTI0EN` reader - DTI CDTI0 Output Generation Enable
pub type Dtogcdti0enR = crate::BitReader;
///Field `DTOGCDTI0EN` writer - DTI CDTI0 Output Generation Enable
pub type Dtogcdti0enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTOGCDTI1EN` reader - DTI CDTI1 Output Generation Enable
pub type Dtogcdti1enR = crate::BitReader;
///Field `DTOGCDTI1EN` writer - DTI CDTI1 Output Generation Enable
pub type Dtogcdti1enW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTOGCDTI2EN` reader - DTI CDTI2 Output Generation Enable
pub type Dtogcdti2enR = crate::BitReader;
///Field `DTOGCDTI2EN` writer - DTI CDTI2 Output Generation Enable
pub type Dtogcdti2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DTI CC0 Output Generation Enable
    #[inline(always)]
    pub fn dtogcc0en(&self) -> Dtogcc0enR {
        Dtogcc0enR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTI CC1 Output Generation Enable
    #[inline(always)]
    pub fn dtogcc1en(&self) -> Dtogcc1enR {
        Dtogcc1enR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTI CC2 Output Generation Enable
    #[inline(always)]
    pub fn dtogcc2en(&self) -> Dtogcc2enR {
        Dtogcc2enR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTI CDTI0 Output Generation Enable
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> Dtogcdti0enR {
        Dtogcdti0enR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DTI CDTI1 Output Generation Enable
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> Dtogcdti1enR {
        Dtogcdti1enR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DTI CDTI2 Output Generation Enable
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> Dtogcdti2enR {
        Dtogcdti2enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTOGEN")
            .field("dtogcc0en", &self.dtogcc0en())
            .field("dtogcc1en", &self.dtogcc1en())
            .field("dtogcc2en", &self.dtogcc2en())
            .field("dtogcdti0en", &self.dtogcdti0en())
            .field("dtogcdti1en", &self.dtogcdti1en())
            .field("dtogcdti2en", &self.dtogcdti2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DTI CC0 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcc0en(&mut self) -> Dtogcc0enW<DTOGENrs> {
        Dtogcc0enW::new(self, 0)
    }
    ///Bit 1 - DTI CC1 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcc1en(&mut self) -> Dtogcc1enW<DTOGENrs> {
        Dtogcc1enW::new(self, 1)
    }
    ///Bit 2 - DTI CC2 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcc2en(&mut self) -> Dtogcc2enW<DTOGENrs> {
        Dtogcc2enW::new(self, 2)
    }
    ///Bit 3 - DTI CDTI0 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti0en(&mut self) -> Dtogcdti0enW<DTOGENrs> {
        Dtogcdti0enW::new(self, 3)
    }
    ///Bit 4 - DTI CDTI1 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti1en(&mut self) -> Dtogcdti1enW<DTOGENrs> {
        Dtogcdti1enW::new(self, 4)
    }
    ///Bit 5 - DTI CDTI2 Output Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn dtogcdti2en(&mut self) -> Dtogcdti2enW<DTOGENrs> {
        Dtogcdti2enW::new(self, 5)
    }
}
///DTI Output Generation Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`dtogen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtogen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTOGENrs;
impl crate::RegisterSpec for DTOGENrs {
    type Ux = u32;
}
///`read()` method returns [`dtogen::R`](R) reader structure
impl crate::Readable for DTOGENrs {}
///`write(|w| ..)` method takes [`dtogen::W`](W) writer structure
impl crate::Writable for DTOGENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTOGEN to value 0
impl crate::Resettable for DTOGENrs {
    const RESET_VALUE: u32 = 0;
}
