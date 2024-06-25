#[doc = "Register `INPUT` reader"]
pub type R = crate::R<INPUTrs>;
#[doc = "Register `INPUT` writer"]
pub type W = crate::W<INPUTrs>;
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPRSSEL {
    #[doc = "0: PRS Channel 0 selected"]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected"]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected"]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected"]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected"]
    Prsch11 = 11,
}
impl From<RXPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: RXPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for RXPRSSEL {}
#[doc = "Field `RXPRSSEL` reader - RX PRS Channel Select"]
pub type RxprsselR = crate::FieldReader<RXPRSSEL>;
impl RxprsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXPRSSEL> {
        match self.bits {
            0 => Some(RXPRSSEL::Prsch0),
            1 => Some(RXPRSSEL::Prsch1),
            2 => Some(RXPRSSEL::Prsch2),
            3 => Some(RXPRSSEL::Prsch3),
            4 => Some(RXPRSSEL::Prsch4),
            5 => Some(RXPRSSEL::Prsch5),
            6 => Some(RXPRSSEL::Prsch6),
            7 => Some(RXPRSSEL::Prsch7),
            8 => Some(RXPRSSEL::Prsch8),
            9 => Some(RXPRSSEL::Prsch9),
            10 => Some(RXPRSSEL::Prsch10),
            11 => Some(RXPRSSEL::Prsch11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSEL::Prsch11
    }
}
#[doc = "Field `RXPRSSEL` writer - RX PRS Channel Select"]
pub type RxprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, RXPRSSEL>;
impl<'a, REG> RxprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch11)
    }
}
#[doc = "Field `RXPRS` reader - PRS RX Enable"]
pub type RxprsR = crate::BitReader;
#[doc = "Field `RXPRS` writer - PRS RX Enable"]
pub type RxprsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RxprsselR {
        RxprsselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RxprsR {
        RxprsR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("rxprssel", &self.rxprssel())
            .field("rxprs", &self.rxprs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RX PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxprssel(&mut self) -> RxprsselW<INPUTrs> {
        RxprsselW::new(self, 0)
    }
    #[doc = "Bit 5 - PRS RX Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxprs(&mut self) -> RxprsW<INPUTrs> {
        RxprsW::new(self, 5)
    }
}
#[doc = "LEUART Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUTrs;
impl crate::RegisterSpec for INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input::R`](R) reader structure"]
impl crate::Readable for INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`input::W`](W) writer structure"]
impl crate::Writable for INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUTrs {
    const RESET_VALUE: u32 = 0;
}
