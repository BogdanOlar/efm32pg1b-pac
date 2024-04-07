#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0rs>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0rs>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<RXLOC> for u8 {
    #[inline(always)]
    fn from(variant: RXLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXLOC {
    type Ux = u8;
}
impl crate::IsEnum for RXLOC {}
#[doc = "Field `RXLOC` reader - I/O Location"]
pub type RxlocR = crate::FieldReader<RXLOC>;
impl RxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXLOC> {
        match self.bits {
            0 => Some(RXLOC::Loc0),
            1 => Some(RXLOC::Loc1),
            2 => Some(RXLOC::Loc2),
            3 => Some(RXLOC::Loc3),
            4 => Some(RXLOC::Loc4),
            5 => Some(RXLOC::Loc5),
            6 => Some(RXLOC::Loc6),
            7 => Some(RXLOC::Loc7),
            8 => Some(RXLOC::Loc8),
            9 => Some(RXLOC::Loc9),
            10 => Some(RXLOC::Loc10),
            11 => Some(RXLOC::Loc11),
            12 => Some(RXLOC::Loc12),
            13 => Some(RXLOC::Loc13),
            14 => Some(RXLOC::Loc14),
            15 => Some(RXLOC::Loc15),
            16 => Some(RXLOC::Loc16),
            17 => Some(RXLOC::Loc17),
            18 => Some(RXLOC::Loc18),
            19 => Some(RXLOC::Loc19),
            20 => Some(RXLOC::Loc20),
            21 => Some(RXLOC::Loc21),
            22 => Some(RXLOC::Loc22),
            23 => Some(RXLOC::Loc23),
            24 => Some(RXLOC::Loc24),
            25 => Some(RXLOC::Loc25),
            26 => Some(RXLOC::Loc26),
            27 => Some(RXLOC::Loc27),
            28 => Some(RXLOC::Loc28),
            29 => Some(RXLOC::Loc29),
            30 => Some(RXLOC::Loc30),
            31 => Some(RXLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == RXLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == RXLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == RXLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == RXLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == RXLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == RXLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == RXLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == RXLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == RXLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == RXLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == RXLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == RXLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == RXLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == RXLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == RXLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == RXLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == RXLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == RXLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == RXLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == RXLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == RXLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == RXLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == RXLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == RXLOC::Loc31
    }
}
#[doc = "Field `RXLOC` writer - I/O Location"]
pub type RxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, RXLOC>;
impl<'a, REG> RxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(RXLOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<TXLOC> for u8 {
    #[inline(always)]
    fn from(variant: TXLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXLOC {
    type Ux = u8;
}
impl crate::IsEnum for TXLOC {}
#[doc = "Field `TXLOC` reader - I/O Location"]
pub type TxlocR = crate::FieldReader<TXLOC>;
impl TxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXLOC> {
        match self.bits {
            0 => Some(TXLOC::Loc0),
            1 => Some(TXLOC::Loc1),
            2 => Some(TXLOC::Loc2),
            3 => Some(TXLOC::Loc3),
            4 => Some(TXLOC::Loc4),
            5 => Some(TXLOC::Loc5),
            6 => Some(TXLOC::Loc6),
            7 => Some(TXLOC::Loc7),
            8 => Some(TXLOC::Loc8),
            9 => Some(TXLOC::Loc9),
            10 => Some(TXLOC::Loc10),
            11 => Some(TXLOC::Loc11),
            12 => Some(TXLOC::Loc12),
            13 => Some(TXLOC::Loc13),
            14 => Some(TXLOC::Loc14),
            15 => Some(TXLOC::Loc15),
            16 => Some(TXLOC::Loc16),
            17 => Some(TXLOC::Loc17),
            18 => Some(TXLOC::Loc18),
            19 => Some(TXLOC::Loc19),
            20 => Some(TXLOC::Loc20),
            21 => Some(TXLOC::Loc21),
            22 => Some(TXLOC::Loc22),
            23 => Some(TXLOC::Loc23),
            24 => Some(TXLOC::Loc24),
            25 => Some(TXLOC::Loc25),
            26 => Some(TXLOC::Loc26),
            27 => Some(TXLOC::Loc27),
            28 => Some(TXLOC::Loc28),
            29 => Some(TXLOC::Loc29),
            30 => Some(TXLOC::Loc30),
            31 => Some(TXLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == TXLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == TXLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == TXLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == TXLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == TXLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == TXLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == TXLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == TXLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == TXLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == TXLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == TXLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == TXLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == TXLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == TXLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == TXLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == TXLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == TXLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == TXLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == TXLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == TXLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == TXLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == TXLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == TXLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == TXLOC::Loc31
    }
}
#[doc = "Field `TXLOC` writer - I/O Location"]
pub type TxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, TXLOC>;
impl<'a, REG> TxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(TXLOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<CSLOC> for u8 {
    #[inline(always)]
    fn from(variant: CSLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSLOC {
    type Ux = u8;
}
impl crate::IsEnum for CSLOC {}
#[doc = "Field `CSLOC` reader - I/O Location"]
pub type CslocR = crate::FieldReader<CSLOC>;
impl CslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CSLOC> {
        match self.bits {
            0 => Some(CSLOC::Loc0),
            1 => Some(CSLOC::Loc1),
            2 => Some(CSLOC::Loc2),
            3 => Some(CSLOC::Loc3),
            4 => Some(CSLOC::Loc4),
            5 => Some(CSLOC::Loc5),
            6 => Some(CSLOC::Loc6),
            7 => Some(CSLOC::Loc7),
            8 => Some(CSLOC::Loc8),
            9 => Some(CSLOC::Loc9),
            10 => Some(CSLOC::Loc10),
            11 => Some(CSLOC::Loc11),
            12 => Some(CSLOC::Loc12),
            13 => Some(CSLOC::Loc13),
            14 => Some(CSLOC::Loc14),
            15 => Some(CSLOC::Loc15),
            16 => Some(CSLOC::Loc16),
            17 => Some(CSLOC::Loc17),
            18 => Some(CSLOC::Loc18),
            19 => Some(CSLOC::Loc19),
            20 => Some(CSLOC::Loc20),
            21 => Some(CSLOC::Loc21),
            22 => Some(CSLOC::Loc22),
            23 => Some(CSLOC::Loc23),
            24 => Some(CSLOC::Loc24),
            25 => Some(CSLOC::Loc25),
            26 => Some(CSLOC::Loc26),
            27 => Some(CSLOC::Loc27),
            28 => Some(CSLOC::Loc28),
            29 => Some(CSLOC::Loc29),
            30 => Some(CSLOC::Loc30),
            31 => Some(CSLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CSLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CSLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CSLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CSLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CSLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CSLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CSLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CSLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CSLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CSLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CSLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CSLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CSLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CSLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CSLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CSLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CSLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CSLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CSLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CSLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CSLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CSLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CSLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CSLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CSLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CSLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CSLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CSLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CSLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CSLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CSLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CSLOC::Loc31
    }
}
#[doc = "Field `CSLOC` writer - I/O Location"]
pub type CslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, CSLOC>;
impl<'a, REG> CslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CSLOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<CLKLOC> for u8 {
    #[inline(always)]
    fn from(variant: CLKLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKLOC {
    type Ux = u8;
}
impl crate::IsEnum for CLKLOC {}
#[doc = "Field `CLKLOC` reader - I/O Location"]
pub type ClklocR = crate::FieldReader<CLKLOC>;
impl ClklocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKLOC> {
        match self.bits {
            0 => Some(CLKLOC::Loc0),
            1 => Some(CLKLOC::Loc1),
            2 => Some(CLKLOC::Loc2),
            3 => Some(CLKLOC::Loc3),
            4 => Some(CLKLOC::Loc4),
            5 => Some(CLKLOC::Loc5),
            6 => Some(CLKLOC::Loc6),
            7 => Some(CLKLOC::Loc7),
            8 => Some(CLKLOC::Loc8),
            9 => Some(CLKLOC::Loc9),
            10 => Some(CLKLOC::Loc10),
            11 => Some(CLKLOC::Loc11),
            12 => Some(CLKLOC::Loc12),
            13 => Some(CLKLOC::Loc13),
            14 => Some(CLKLOC::Loc14),
            15 => Some(CLKLOC::Loc15),
            16 => Some(CLKLOC::Loc16),
            17 => Some(CLKLOC::Loc17),
            18 => Some(CLKLOC::Loc18),
            19 => Some(CLKLOC::Loc19),
            20 => Some(CLKLOC::Loc20),
            21 => Some(CLKLOC::Loc21),
            22 => Some(CLKLOC::Loc22),
            23 => Some(CLKLOC::Loc23),
            24 => Some(CLKLOC::Loc24),
            25 => Some(CLKLOC::Loc25),
            26 => Some(CLKLOC::Loc26),
            27 => Some(CLKLOC::Loc27),
            28 => Some(CLKLOC::Loc28),
            29 => Some(CLKLOC::Loc29),
            30 => Some(CLKLOC::Loc30),
            31 => Some(CLKLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CLKLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CLKLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CLKLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CLKLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CLKLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CLKLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CLKLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CLKLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CLKLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CLKLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CLKLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CLKLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CLKLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CLKLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CLKLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CLKLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CLKLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CLKLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CLKLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CLKLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CLKLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CLKLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CLKLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CLKLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CLKLOC::Loc31
    }
}
#[doc = "Field `CLKLOC` writer - I/O Location"]
pub type ClklocW<'a, REG> = crate::FieldWriter<'a, REG, 6, CLKLOC>;
impl<'a, REG> ClklocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CLKLOC::Loc31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn rxloc(&self) -> RxlocR {
        RxlocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn txloc(&self) -> TxlocR {
        TxlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn csloc(&self) -> CslocR {
        CslocR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn clkloc(&self) -> ClklocR {
        ClklocR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn rxloc(&mut self) -> RxlocW<ROUTELOC0rs> {
        RxlocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn txloc(&mut self) -> TxlocW<ROUTELOC0rs> {
        TxlocW::new(self, 8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn csloc(&mut self) -> CslocW<ROUTELOC0rs> {
        CslocW::new(self, 16)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn clkloc(&mut self) -> ClklocW<ROUTELOC0rs> {
        ClklocW::new(self, 24)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0rs {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
