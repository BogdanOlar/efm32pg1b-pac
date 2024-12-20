///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `SEVONPRS` reader - Set Event on PRS
pub type SevonprsR = crate::BitReader;
///Field `SEVONPRS` writer - Set Event on PRS
pub type SevonprsW<'a, REG> = crate::BitWriter<'a, REG>;
///SEVONPRS PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEVONPRSSEL {
    ///0: PRS Channel 0 selected
    Prsch0 = 0,
    ///1: PRS Channel 1 selected
    Prsch1 = 1,
    ///2: PRS Channel 2 selected
    Prsch2 = 2,
    ///3: PRS Channel 3 selected
    Prsch3 = 3,
    ///4: PRS Channel 4 selected
    Prsch4 = 4,
    ///5: PRS Channel 5 selected
    Prsch5 = 5,
    ///6: PRS Channel 6 selected
    Prsch6 = 6,
    ///7: PRS Channel 7 selected
    Prsch7 = 7,
    ///8: PRS Channel 8 selected
    Prsch8 = 8,
    ///9: PRS Channel 9 selected
    Prsch9 = 9,
    ///10: PRS Channel 10 selected
    Prsch10 = 10,
    ///11: PRS Channel 11 selected
    Prsch11 = 11,
}
impl From<SEVONPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: SEVONPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEVONPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for SEVONPRSSEL {}
///Field `SEVONPRSSEL` reader - SEVONPRS PRS Channel Select
pub type SevonprsselR = crate::FieldReader<SEVONPRSSEL>;
impl SevonprsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SEVONPRSSEL> {
        match self.bits {
            0 => Some(SEVONPRSSEL::Prsch0),
            1 => Some(SEVONPRSSEL::Prsch1),
            2 => Some(SEVONPRSSEL::Prsch2),
            3 => Some(SEVONPRSSEL::Prsch3),
            4 => Some(SEVONPRSSEL::Prsch4),
            5 => Some(SEVONPRSSEL::Prsch5),
            6 => Some(SEVONPRSSEL::Prsch6),
            7 => Some(SEVONPRSSEL::Prsch7),
            8 => Some(SEVONPRSSEL::Prsch8),
            9 => Some(SEVONPRSSEL::Prsch9),
            10 => Some(SEVONPRSSEL::Prsch10),
            11 => Some(SEVONPRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == SEVONPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == SEVONPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == SEVONPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == SEVONPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == SEVONPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == SEVONPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == SEVONPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == SEVONPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == SEVONPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == SEVONPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == SEVONPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == SEVONPRSSEL::Prsch11
    }
}
///Field `SEVONPRSSEL` writer - SEVONPRS PRS Channel Select
pub type SevonprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, SEVONPRSSEL>;
impl<'a, REG> SevonprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(SEVONPRSSEL::Prsch11)
    }
}
impl R {
    ///Bit 0 - Set Event on PRS
    #[inline(always)]
    pub fn sevonprs(&self) -> SevonprsR {
        SevonprsR::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - SEVONPRS PRS Channel Select
    #[inline(always)]
    pub fn sevonprssel(&self) -> SevonprsselR {
        SevonprsselR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sevonprs", &self.sevonprs())
            .field("sevonprssel", &self.sevonprssel())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set Event on PRS
    #[inline(always)]
    #[must_use]
    pub fn sevonprs(&mut self) -> SevonprsW<CTRLrs> {
        SevonprsW::new(self, 0)
    }
    ///Bits 1:4 - SEVONPRS PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn sevonprssel(&mut self) -> SevonprsselW<CTRLrs> {
        SevonprsselW::new(self, 1)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
