///Register `INPUT` reader
pub type R = crate::R<INPUTrs>;
///Register `INPUT` writer
pub type W = crate::W<INPUTrs>;
///RX PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPRSSEL {
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
///Field `RXPRSSEL` reader - RX PRS Channel Select
pub type RxprsselR = crate::FieldReader<RXPRSSEL>;
impl RxprsselR {
    ///Get enumerated values variant
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
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == RXPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == RXPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == RXPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == RXPRSSEL::Prsch11
    }
}
///Field `RXPRSSEL` writer - RX PRS Channel Select
pub type RxprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, RXPRSSEL>;
impl<'a, REG> RxprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(RXPRSSEL::Prsch11)
    }
}
///Field `RXPRS` reader - PRS RX Enable
pub type RxprsR = crate::BitReader;
///Field `RXPRS` writer - PRS RX Enable
pub type RxprsW<'a, REG> = crate::BitWriter<'a, REG>;
///CLK PRS Channel Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKPRSSEL {
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
impl From<CLKPRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: CLKPRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKPRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for CLKPRSSEL {}
///Field `CLKPRSSEL` reader - CLK PRS Channel Select
pub type ClkprsselR = crate::FieldReader<CLKPRSSEL>;
impl ClkprsselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKPRSSEL> {
        match self.bits {
            0 => Some(CLKPRSSEL::Prsch0),
            1 => Some(CLKPRSSEL::Prsch1),
            2 => Some(CLKPRSSEL::Prsch2),
            3 => Some(CLKPRSSEL::Prsch3),
            4 => Some(CLKPRSSEL::Prsch4),
            5 => Some(CLKPRSSEL::Prsch5),
            6 => Some(CLKPRSSEL::Prsch6),
            7 => Some(CLKPRSSEL::Prsch7),
            8 => Some(CLKPRSSEL::Prsch8),
            9 => Some(CLKPRSSEL::Prsch9),
            10 => Some(CLKPRSSEL::Prsch10),
            11 => Some(CLKPRSSEL::Prsch11),
            _ => None,
        }
    }
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == CLKPRSSEL::Prsch0
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == CLKPRSSEL::Prsch1
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == CLKPRSSEL::Prsch2
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == CLKPRSSEL::Prsch3
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == CLKPRSSEL::Prsch4
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == CLKPRSSEL::Prsch5
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == CLKPRSSEL::Prsch6
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == CLKPRSSEL::Prsch7
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == CLKPRSSEL::Prsch8
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == CLKPRSSEL::Prsch9
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == CLKPRSSEL::Prsch10
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == CLKPRSSEL::Prsch11
    }
}
///Field `CLKPRSSEL` writer - CLK PRS Channel Select
pub type ClkprsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKPRSSEL>;
impl<'a, REG> ClkprsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PRS Channel 0 selected
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch0)
    }
    ///PRS Channel 1 selected
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch1)
    }
    ///PRS Channel 2 selected
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch2)
    }
    ///PRS Channel 3 selected
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch3)
    }
    ///PRS Channel 4 selected
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch4)
    }
    ///PRS Channel 5 selected
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch5)
    }
    ///PRS Channel 6 selected
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch6)
    }
    ///PRS Channel 7 selected
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch7)
    }
    ///PRS Channel 8 selected
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch8)
    }
    ///PRS Channel 9 selected
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch9)
    }
    ///PRS Channel 10 selected
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch10)
    }
    ///PRS Channel 11 selected
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPRSSEL::Prsch11)
    }
}
///Field `CLKPRS` reader - PRS CLK Enable
pub type ClkprsR = crate::BitReader;
///Field `CLKPRS` writer - PRS CLK Enable
pub type ClkprsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - RX PRS Channel Select
    #[inline(always)]
    pub fn rxprssel(&self) -> RxprsselR {
        RxprsselR::new((self.bits & 0x0f) as u8)
    }
    ///Bit 7 - PRS RX Enable
    #[inline(always)]
    pub fn rxprs(&self) -> RxprsR {
        RxprsR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - CLK PRS Channel Select
    #[inline(always)]
    pub fn clkprssel(&self) -> ClkprsselR {
        ClkprsselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 15 - PRS CLK Enable
    #[inline(always)]
    pub fn clkprs(&self) -> ClkprsR {
        ClkprsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUT")
            .field("rxprssel", &self.rxprssel())
            .field("rxprs", &self.rxprs())
            .field("clkprssel", &self.clkprssel())
            .field("clkprs", &self.clkprs())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - RX PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn rxprssel(&mut self) -> RxprsselW<INPUTrs> {
        RxprsselW::new(self, 0)
    }
    ///Bit 7 - PRS RX Enable
    #[inline(always)]
    #[must_use]
    pub fn rxprs(&mut self) -> RxprsW<INPUTrs> {
        RxprsW::new(self, 7)
    }
    ///Bits 8:11 - CLK PRS Channel Select
    #[inline(always)]
    #[must_use]
    pub fn clkprssel(&mut self) -> ClkprsselW<INPUTrs> {
        ClkprsselW::new(self, 8)
    }
    ///Bit 15 - PRS CLK Enable
    #[inline(always)]
    #[must_use]
    pub fn clkprs(&mut self) -> ClkprsW<INPUTrs> {
        ClkprsW::new(self, 15)
    }
}
///USART Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTrs;
impl crate::RegisterSpec for INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`input::R`](R) reader structure
impl crate::Readable for INPUTrs {}
///`write(|w| ..)` method takes [`input::W`](W) writer structure
impl crate::Writable for INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUT to value 0
impl crate::Resettable for INPUTrs {
    const RESET_VALUE: u32 = 0;
}
