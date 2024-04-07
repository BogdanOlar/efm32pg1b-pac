#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `EN` reader - Current DAC Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Current DAC Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURSINK` reader - Current Sink Enable"]
pub type CursinkR = crate::BitReader;
#[doc = "Field `CURSINK` writer - Current Sink Enable"]
pub type CursinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINOUTTRANS` reader - Minimum Output Transition Enable"]
pub type MinouttransR = crate::BitReader;
#[doc = "Field `MINOUTTRANS` writer - Minimum Output Transition Enable"]
pub type MinouttransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTOUTEN` reader - APORT Output Enable"]
pub type AportoutenR = crate::BitReader;
#[doc = "Field `APORTOUTEN` writer - APORT Output Enable"]
pub type AportoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "APORT Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APORTOUTSEL {
    #[doc = "32: APORT1X Channel 0"]
    Aport1xch0 = 32,
    #[doc = "33: APORT1Y Channel 1"]
    Aport1ych1 = 33,
    #[doc = "34: APORT1X Channel 2"]
    Aport1xch2 = 34,
    #[doc = "35: APORT1Y Channel 3"]
    Aport1ych3 = 35,
    #[doc = "36: APORT1X Channel 4"]
    Aport1xch4 = 36,
    #[doc = "37: APORT1Y Channel 5"]
    Aport1ych5 = 37,
    #[doc = "38: APORT1X Channel 6"]
    Aport1xch6 = 38,
    #[doc = "39: APORT1Y Channel 7"]
    Aport1ych7 = 39,
    #[doc = "40: APORT1X Channel 8"]
    Aport1xch8 = 40,
    #[doc = "41: APORT1Y Channel 9"]
    Aport1ych9 = 41,
    #[doc = "42: APORT1X Channel 10"]
    Aport1xch10 = 42,
    #[doc = "43: APORT1Y Channel 11"]
    Aport1ych11 = 43,
    #[doc = "44: APORT1X Channel 12"]
    Aport1xch12 = 44,
    #[doc = "45: APORT1Y Channel 13"]
    Aport1ych13 = 45,
    #[doc = "46: APORT1X Channel 14"]
    Aport1xch14 = 46,
    #[doc = "47: APORT1Y Channel 15"]
    Aport1ych15 = 47,
    #[doc = "48: APORT1X Channel 16"]
    Aport1xch16 = 48,
    #[doc = "49: APORT1Y Channel 17"]
    Aport1ych17 = 49,
    #[doc = "50: APORT1X Channel 18"]
    Aport1xch18 = 50,
    #[doc = "51: APORT1Y Channel 19"]
    Aport1ych19 = 51,
    #[doc = "52: APORT1X Channel 20"]
    Aport1xch20 = 52,
    #[doc = "53: APORT1Y Channel 21"]
    Aport1ych21 = 53,
    #[doc = "54: APORT1X Channel 22"]
    Aport1xch22 = 54,
    #[doc = "55: APORT1Y Channel 23"]
    Aport1ych23 = 55,
    #[doc = "56: APORT1X Channel 24"]
    Aport1xch24 = 56,
    #[doc = "57: APORT1Y Channel 25"]
    Aport1ych25 = 57,
    #[doc = "58: APORT1X Channel 26"]
    Aport1xch26 = 58,
    #[doc = "59: APORT1Y Channel 27"]
    Aport1ych27 = 59,
    #[doc = "60: APORT1X Channel 28"]
    Aport1xch28 = 60,
    #[doc = "61: APORT1Y Channel 29"]
    Aport1ych29 = 61,
    #[doc = "62: APORT1X Channel 30"]
    Aport1xch30 = 62,
    #[doc = "63: APORT1Y Channel 31"]
    Aport1ych31 = 63,
}
impl From<APORTOUTSEL> for u8 {
    #[inline(always)]
    fn from(variant: APORTOUTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APORTOUTSEL {
    type Ux = u8;
}
impl crate::IsEnum for APORTOUTSEL {}
#[doc = "Field `APORTOUTSEL` reader - APORT Output Select"]
pub type AportoutselR = crate::FieldReader<APORTOUTSEL>;
impl AportoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<APORTOUTSEL> {
        match self.bits {
            32 => Some(APORTOUTSEL::Aport1xch0),
            33 => Some(APORTOUTSEL::Aport1ych1),
            34 => Some(APORTOUTSEL::Aport1xch2),
            35 => Some(APORTOUTSEL::Aport1ych3),
            36 => Some(APORTOUTSEL::Aport1xch4),
            37 => Some(APORTOUTSEL::Aport1ych5),
            38 => Some(APORTOUTSEL::Aport1xch6),
            39 => Some(APORTOUTSEL::Aport1ych7),
            40 => Some(APORTOUTSEL::Aport1xch8),
            41 => Some(APORTOUTSEL::Aport1ych9),
            42 => Some(APORTOUTSEL::Aport1xch10),
            43 => Some(APORTOUTSEL::Aport1ych11),
            44 => Some(APORTOUTSEL::Aport1xch12),
            45 => Some(APORTOUTSEL::Aport1ych13),
            46 => Some(APORTOUTSEL::Aport1xch14),
            47 => Some(APORTOUTSEL::Aport1ych15),
            48 => Some(APORTOUTSEL::Aport1xch16),
            49 => Some(APORTOUTSEL::Aport1ych17),
            50 => Some(APORTOUTSEL::Aport1xch18),
            51 => Some(APORTOUTSEL::Aport1ych19),
            52 => Some(APORTOUTSEL::Aport1xch20),
            53 => Some(APORTOUTSEL::Aport1ych21),
            54 => Some(APORTOUTSEL::Aport1xch22),
            55 => Some(APORTOUTSEL::Aport1ych23),
            56 => Some(APORTOUTSEL::Aport1xch24),
            57 => Some(APORTOUTSEL::Aport1ych25),
            58 => Some(APORTOUTSEL::Aport1xch26),
            59 => Some(APORTOUTSEL::Aport1ych27),
            60 => Some(APORTOUTSEL::Aport1xch28),
            61 => Some(APORTOUTSEL::Aport1ych29),
            62 => Some(APORTOUTSEL::Aport1xch30),
            63 => Some(APORTOUTSEL::Aport1ych31),
            _ => None,
        }
    }
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch0
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych1
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch2
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych3
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch4
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych5
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch6
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych7
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch8
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych9
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch10
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych11
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch12
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych13
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch14
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych15
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch16
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych17
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch18
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych19
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch20
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych21
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch22
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych23
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch24
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych25
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch26
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych27
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch28
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych29
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == APORTOUTSEL::Aport1xch30
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == APORTOUTSEL::Aport1ych31
    }
}
#[doc = "Field `APORTOUTSEL` writer - APORT Output Select"]
pub type AportoutselW<'a, REG> = crate::FieldWriter<'a, REG, 8, APORTOUTSEL>;
impl<'a, REG> AportoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APORT1X Channel 0"]
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch0)
    }
    #[doc = "APORT1Y Channel 1"]
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych1)
    }
    #[doc = "APORT1X Channel 2"]
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch2)
    }
    #[doc = "APORT1Y Channel 3"]
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych3)
    }
    #[doc = "APORT1X Channel 4"]
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch4)
    }
    #[doc = "APORT1Y Channel 5"]
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych5)
    }
    #[doc = "APORT1X Channel 6"]
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch6)
    }
    #[doc = "APORT1Y Channel 7"]
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych7)
    }
    #[doc = "APORT1X Channel 8"]
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch8)
    }
    #[doc = "APORT1Y Channel 9"]
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych9)
    }
    #[doc = "APORT1X Channel 10"]
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch10)
    }
    #[doc = "APORT1Y Channel 11"]
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych11)
    }
    #[doc = "APORT1X Channel 12"]
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch12)
    }
    #[doc = "APORT1Y Channel 13"]
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych13)
    }
    #[doc = "APORT1X Channel 14"]
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch14)
    }
    #[doc = "APORT1Y Channel 15"]
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych15)
    }
    #[doc = "APORT1X Channel 16"]
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch16)
    }
    #[doc = "APORT1Y Channel 17"]
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych17)
    }
    #[doc = "APORT1X Channel 18"]
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch18)
    }
    #[doc = "APORT1Y Channel 19"]
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych19)
    }
    #[doc = "APORT1X Channel 20"]
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch20)
    }
    #[doc = "APORT1Y Channel 21"]
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych21)
    }
    #[doc = "APORT1X Channel 22"]
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch22)
    }
    #[doc = "APORT1Y Channel 23"]
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych23)
    }
    #[doc = "APORT1X Channel 24"]
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch24)
    }
    #[doc = "APORT1Y Channel 25"]
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych25)
    }
    #[doc = "APORT1X Channel 26"]
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch26)
    }
    #[doc = "APORT1Y Channel 27"]
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych27)
    }
    #[doc = "APORT1X Channel 28"]
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch28)
    }
    #[doc = "APORT1Y Channel 29"]
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych29)
    }
    #[doc = "APORT1X Channel 30"]
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1xch30)
    }
    #[doc = "APORT1Y Channel 31"]
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut crate::W<REG> {
        self.variant(APORTOUTSEL::Aport1ych31)
    }
}
#[doc = "Field `PWRSEL` reader - Power Select"]
pub type PwrselR = crate::BitReader;
#[doc = "Field `PWRSEL` writer - Power Select"]
pub type PwrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2DELAY` reader - EM2 Delay"]
pub type Em2delayR = crate::BitReader;
#[doc = "Field `EM2DELAY` writer - EM2 Delay"]
pub type Em2delayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTMASTERDIS` reader - APORT Bus Master Disable"]
pub type AportmasterdisR = crate::BitReader;
#[doc = "Field `APORTMASTERDIS` writer - APORT Bus Master Disable"]
pub type AportmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTOUTENPRS` reader - PRS Controlled APORT Output Enable"]
pub type AportoutenprsR = crate::BitReader;
#[doc = "Field `APORTOUTENPRS` writer - PRS Controlled APORT Output Enable"]
pub type AportoutenprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IDAC Output Enable PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL {
    #[doc = "0: PRS Channel 0 selected."]
    Prsch0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    Prsch1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    Prsch2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    Prsch3 = 3,
    #[doc = "4: PRS Channel 4 selected."]
    Prsch4 = 4,
    #[doc = "5: PRS Channel 5 selected."]
    Prsch5 = 5,
    #[doc = "6: PRS Channel 6 selected."]
    Prsch6 = 6,
    #[doc = "7: PRS Channel 7 selected."]
    Prsch7 = 7,
    #[doc = "8: PRS Channel 8 selected."]
    Prsch8 = 8,
    #[doc = "9: PRS Channel 9 selected."]
    Prsch9 = 9,
    #[doc = "10: PRS Channel 10 selected."]
    Prsch10 = 10,
    #[doc = "11: PRS Channel 11 selected."]
    Prsch11 = 11,
}
impl From<PRSSEL> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRSSEL {
    type Ux = u8;
}
impl crate::IsEnum for PRSSEL {}
#[doc = "Field `PRSSEL` reader - IDAC Output Enable PRS Channel Select"]
pub type PrsselR = crate::FieldReader<PRSSEL>;
impl PrsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRSSEL> {
        match self.bits {
            0 => Some(PRSSEL::Prsch0),
            1 => Some(PRSSEL::Prsch1),
            2 => Some(PRSSEL::Prsch2),
            3 => Some(PRSSEL::Prsch3),
            4 => Some(PRSSEL::Prsch4),
            5 => Some(PRSSEL::Prsch5),
            6 => Some(PRSSEL::Prsch6),
            7 => Some(PRSSEL::Prsch7),
            8 => Some(PRSSEL::Prsch8),
            9 => Some(PRSSEL::Prsch9),
            10 => Some(PRSSEL::Prsch10),
            11 => Some(PRSSEL::Prsch11),
            _ => None,
        }
    }
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL::Prsch0
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL::Prsch1
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL::Prsch2
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL::Prsch3
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL::Prsch4
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL::Prsch5
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL::Prsch6
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL::Prsch7
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL::Prsch8
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL::Prsch9
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL::Prsch10
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL::Prsch11
    }
}
#[doc = "Field `PRSSEL` writer - IDAC Output Enable PRS Channel Select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4, PRSSEL>;
impl<'a, REG> PrsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch3)
    }
    #[doc = "PRS Channel 4 selected."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch4)
    }
    #[doc = "PRS Channel 5 selected."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch5)
    }
    #[doc = "PRS Channel 6 selected."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch6)
    }
    #[doc = "PRS Channel 7 selected."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch7)
    }
    #[doc = "PRS Channel 8 selected."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch8)
    }
    #[doc = "PRS Channel 9 selected."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch9)
    }
    #[doc = "PRS Channel 10 selected."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch10)
    }
    #[doc = "PRS Channel 11 selected."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut crate::W<REG> {
        self.variant(PRSSEL::Prsch11)
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CursinkR {
        CursinkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MinouttransR {
        MinouttransR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> AportoutenR {
        AportoutenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> AportoutselR {
        AportoutselR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PwrselR {
        PwrselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    pub fn em2delay(&self) -> Em2delayR {
        Em2delayR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    pub fn aportmasterdis(&self) -> AportmasterdisR {
        AportmasterdisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    pub fn aportoutenprs(&self) -> AportoutenprsR {
        AportoutenprsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CTRLrs> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cursink(&mut self) -> CursinkW<CTRLrs> {
        CursinkW::new(self, 1)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minouttrans(&mut self) -> MinouttransW<CTRLrs> {
        MinouttransW::new(self, 2)
    }
    #[doc = "Bit 3 - APORT Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportouten(&mut self) -> AportoutenW<CTRLrs> {
        AportoutenW::new(self, 3)
    }
    #[doc = "Bits 4:11 - APORT Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn aportoutsel(&mut self) -> AportoutselW<CTRLrs> {
        AportoutselW::new(self, 4)
    }
    #[doc = "Bit 12 - Power Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsel(&mut self) -> PwrselW<CTRLrs> {
        PwrselW::new(self, 12)
    }
    #[doc = "Bit 13 - EM2 Delay"]
    #[inline(always)]
    #[must_use]
    pub fn em2delay(&mut self) -> Em2delayW<CTRLrs> {
        Em2delayW::new(self, 13)
    }
    #[doc = "Bit 14 - APORT Bus Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportmasterdis(&mut self) -> AportmasterdisW<CTRLrs> {
        AportmasterdisW::new(self, 14)
    }
    #[doc = "Bit 16 - PRS Controlled APORT Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportoutenprs(&mut self) -> AportoutenprsW<CTRLrs> {
        AportoutenprsW::new(self, 16)
    }
    #[doc = "Bits 20:23 - IDAC Output Enable PRS Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<CTRLrs> {
        PrsselW::new(self, 20)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
