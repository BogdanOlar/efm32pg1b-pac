#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UfR = crate::BitReader;
#[doc = "Field `DIRCHG` reader - Direction Change Detect Interrupt Flag"]
pub type DirchgR = crate::BitReader;
#[doc = "Field `CC0` reader - CC Channel 0 Interrupt Flag"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC1` reader - CC Channel 1 Interrupt Flag"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC2` reader - CC Channel 2 Interrupt Flag"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC3` reader - CC Channel 3 Interrupt Flag"]
pub type Cc3R = crate::BitReader;
#[doc = "Field `ICBOF0` reader - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
pub type Icbof0R = crate::BitReader;
#[doc = "Field `ICBOF1` reader - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
pub type Icbof1R = crate::BitReader;
#[doc = "Field `ICBOF2` reader - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
pub type Icbof2R = crate::BitReader;
#[doc = "Field `ICBOF3` reader - CC Channel 3 Input Capture Buffer Overflow Interrupt Flag"]
pub type Icbof3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC Channel 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&self) -> Icbof0R {
        Icbof0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&self) -> Icbof1R {
        Icbof1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&self) -> Icbof2R {
        Icbof2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC Channel 3 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof3(&self) -> Icbof3R {
        Icbof3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("of", &self.of())
            .field("uf", &self.uf())
            .field("dirchg", &self.dirchg())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("cc3", &self.cc3())
            .field("icbof0", &self.icbof0())
            .field("icbof1", &self.icbof1())
            .field("icbof2", &self.icbof2())
            .field("icbof3", &self.icbof3())
            .finish()
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
