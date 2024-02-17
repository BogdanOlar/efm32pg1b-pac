#[doc = "Register `PB_MODEL` reader"]
pub type R = crate::R<PB_MODELrs>;
#[doc = "Register `PB_MODEL` writer"]
pub type W = crate::W<PB_MODELrs>;
#[doc = "Field `MODE0` reader - Pin 0 Mode"]
pub type MODE0_R = crate::FieldReader<MODE0>;
#[doc = "Pin 0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE0> for u8 {
    #[inline(always)]
    fn from(variant: MODE0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0 {
    type Ux = u8;
}
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE0 {
        match self.bits {
            0 => MODE0::Disabled,
            1 => MODE0::Input,
            2 => MODE0::Inputpull,
            3 => MODE0::Inputpullfilter,
            4 => MODE0::Pushpull,
            5 => MODE0::Pushpullalt,
            6 => MODE0::Wiredor,
            7 => MODE0::Wiredorpulldown,
            8 => MODE0::Wiredand,
            9 => MODE0::Wiredandfilter,
            10 => MODE0::Wiredandpullup,
            11 => MODE0::Wiredandpullupfilter,
            12 => MODE0::Wiredandalt,
            13 => MODE0::Wiredandaltfilter,
            14 => MODE0::Wiredandaltpullup,
            15 => MODE0::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE0::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE0::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE0::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE0::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE0::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE0` writer - Pin 0 Mode"]
pub type MODE0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE0>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE1` reader - Pin 1 Mode"]
pub type MODE1_R = crate::FieldReader<MODE1>;
#[doc = "Pin 1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE1> for u8 {
    #[inline(always)]
    fn from(variant: MODE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1 {
    type Ux = u8;
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1 {
        match self.bits {
            0 => MODE1::Disabled,
            1 => MODE1::Input,
            2 => MODE1::Inputpull,
            3 => MODE1::Inputpullfilter,
            4 => MODE1::Pushpull,
            5 => MODE1::Pushpullalt,
            6 => MODE1::Wiredor,
            7 => MODE1::Wiredorpulldown,
            8 => MODE1::Wiredand,
            9 => MODE1::Wiredandfilter,
            10 => MODE1::Wiredandpullup,
            11 => MODE1::Wiredandpullupfilter,
            12 => MODE1::Wiredandalt,
            13 => MODE1::Wiredandaltfilter,
            14 => MODE1::Wiredandaltpullup,
            15 => MODE1::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE1::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE1::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE1::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE1::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE1::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE1::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE1::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE1::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE1::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE1::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE1::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE1::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE1::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE1::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE1::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE1::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE1` writer - Pin 1 Mode"]
pub type MODE1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE1>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE2` reader - Pin 2 Mode"]
pub type MODE2_R = crate::FieldReader<MODE2>;
#[doc = "Pin 2 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE2> for u8 {
    #[inline(always)]
    fn from(variant: MODE2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE2 {
    type Ux = u8;
}
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE2 {
        match self.bits {
            0 => MODE2::Disabled,
            1 => MODE2::Input,
            2 => MODE2::Inputpull,
            3 => MODE2::Inputpullfilter,
            4 => MODE2::Pushpull,
            5 => MODE2::Pushpullalt,
            6 => MODE2::Wiredor,
            7 => MODE2::Wiredorpulldown,
            8 => MODE2::Wiredand,
            9 => MODE2::Wiredandfilter,
            10 => MODE2::Wiredandpullup,
            11 => MODE2::Wiredandpullupfilter,
            12 => MODE2::Wiredandalt,
            13 => MODE2::Wiredandaltfilter,
            14 => MODE2::Wiredandaltpullup,
            15 => MODE2::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE2::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE2::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE2::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE2::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE2::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE2::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE2::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE2::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE2::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE2::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE2::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE2::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE2::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE2::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE2::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE2::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE2` writer - Pin 2 Mode"]
pub type MODE2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE2>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE3` reader - Pin 3 Mode"]
pub type MODE3_R = crate::FieldReader<MODE3>;
#[doc = "Pin 3 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE3> for u8 {
    #[inline(always)]
    fn from(variant: MODE3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE3 {
    type Ux = u8;
}
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE3 {
        match self.bits {
            0 => MODE3::Disabled,
            1 => MODE3::Input,
            2 => MODE3::Inputpull,
            3 => MODE3::Inputpullfilter,
            4 => MODE3::Pushpull,
            5 => MODE3::Pushpullalt,
            6 => MODE3::Wiredor,
            7 => MODE3::Wiredorpulldown,
            8 => MODE3::Wiredand,
            9 => MODE3::Wiredandfilter,
            10 => MODE3::Wiredandpullup,
            11 => MODE3::Wiredandpullupfilter,
            12 => MODE3::Wiredandalt,
            13 => MODE3::Wiredandaltfilter,
            14 => MODE3::Wiredandaltpullup,
            15 => MODE3::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE3::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE3::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE3::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE3::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE3::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE3::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE3::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE3::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE3::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE3::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE3::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE3::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE3::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE3::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE3::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE3::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE3` writer - Pin 3 Mode"]
pub type MODE3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE3>;
impl<'a, REG> MODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE4` reader - Pin 4 Mode"]
pub type MODE4_R = crate::FieldReader<MODE4>;
#[doc = "Pin 4 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE4 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE4> for u8 {
    #[inline(always)]
    fn from(variant: MODE4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE4 {
    type Ux = u8;
}
impl MODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE4 {
        match self.bits {
            0 => MODE4::Disabled,
            1 => MODE4::Input,
            2 => MODE4::Inputpull,
            3 => MODE4::Inputpullfilter,
            4 => MODE4::Pushpull,
            5 => MODE4::Pushpullalt,
            6 => MODE4::Wiredor,
            7 => MODE4::Wiredorpulldown,
            8 => MODE4::Wiredand,
            9 => MODE4::Wiredandfilter,
            10 => MODE4::Wiredandpullup,
            11 => MODE4::Wiredandpullupfilter,
            12 => MODE4::Wiredandalt,
            13 => MODE4::Wiredandaltfilter,
            14 => MODE4::Wiredandaltpullup,
            15 => MODE4::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE4::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE4::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE4::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE4::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE4::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE4::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE4::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE4::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE4::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE4::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE4::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE4::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE4::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE4::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE4::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE4::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE4` writer - Pin 4 Mode"]
pub type MODE4_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE4>;
impl<'a, REG> MODE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE5` reader - Pin 5 Mode"]
pub type MODE5_R = crate::FieldReader<MODE5>;
#[doc = "Pin 5 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE5 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE5> for u8 {
    #[inline(always)]
    fn from(variant: MODE5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE5 {
    type Ux = u8;
}
impl MODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE5 {
        match self.bits {
            0 => MODE5::Disabled,
            1 => MODE5::Input,
            2 => MODE5::Inputpull,
            3 => MODE5::Inputpullfilter,
            4 => MODE5::Pushpull,
            5 => MODE5::Pushpullalt,
            6 => MODE5::Wiredor,
            7 => MODE5::Wiredorpulldown,
            8 => MODE5::Wiredand,
            9 => MODE5::Wiredandfilter,
            10 => MODE5::Wiredandpullup,
            11 => MODE5::Wiredandpullupfilter,
            12 => MODE5::Wiredandalt,
            13 => MODE5::Wiredandaltfilter,
            14 => MODE5::Wiredandaltpullup,
            15 => MODE5::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE5::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE5::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE5::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE5::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE5::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE5::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE5::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE5::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE5::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE5::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE5::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE5::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE5::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE5::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE5::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE5::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE5` writer - Pin 5 Mode"]
pub type MODE5_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE5>;
impl<'a, REG> MODE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE6` reader - Pin 6 Mode"]
pub type MODE6_R = crate::FieldReader<MODE6>;
#[doc = "Pin 6 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE6 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE6> for u8 {
    #[inline(always)]
    fn from(variant: MODE6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE6 {
    type Ux = u8;
}
impl MODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE6 {
        match self.bits {
            0 => MODE6::Disabled,
            1 => MODE6::Input,
            2 => MODE6::Inputpull,
            3 => MODE6::Inputpullfilter,
            4 => MODE6::Pushpull,
            5 => MODE6::Pushpullalt,
            6 => MODE6::Wiredor,
            7 => MODE6::Wiredorpulldown,
            8 => MODE6::Wiredand,
            9 => MODE6::Wiredandfilter,
            10 => MODE6::Wiredandpullup,
            11 => MODE6::Wiredandpullupfilter,
            12 => MODE6::Wiredandalt,
            13 => MODE6::Wiredandaltfilter,
            14 => MODE6::Wiredandaltpullup,
            15 => MODE6::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE6::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE6::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE6::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE6::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE6::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE6::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE6::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE6::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE6::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE6::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE6::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE6::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE6::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE6::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE6::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE6::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE6` writer - Pin 6 Mode"]
pub type MODE6_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE6>;
impl<'a, REG> MODE6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6::Wiredandaltpullupfilter)
    }
}
#[doc = "Field `MODE7` reader - Pin 7 Mode"]
pub type MODE7_R = crate::FieldReader<MODE7>;
#[doc = "Pin 7 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE7 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE7> for u8 {
    #[inline(always)]
    fn from(variant: MODE7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE7 {
    type Ux = u8;
}
impl MODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE7 {
        match self.bits {
            0 => MODE7::Disabled,
            1 => MODE7::Input,
            2 => MODE7::Inputpull,
            3 => MODE7::Inputpullfilter,
            4 => MODE7::Pushpull,
            5 => MODE7::Pushpullalt,
            6 => MODE7::Wiredor,
            7 => MODE7::Wiredorpulldown,
            8 => MODE7::Wiredand,
            9 => MODE7::Wiredandfilter,
            10 => MODE7::Wiredandpullup,
            11 => MODE7::Wiredandpullupfilter,
            12 => MODE7::Wiredandalt,
            13 => MODE7::Wiredandaltfilter,
            14 => MODE7::Wiredandaltpullup,
            15 => MODE7::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE7::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE7::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE7::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE7::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE7::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE7::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE7::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE7::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE7::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE7::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE7::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE7::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE7::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE7::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE7::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE7::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE7` writer - Pin 7 Mode"]
pub type MODE7_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MODE7>;
impl<'a, REG> MODE7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7::Wiredandaltpullupfilter)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> MODE0_W<PB_MODELrs> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<PB_MODELrs> {
        MODE1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<PB_MODELrs> {
        MODE2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> MODE3_W<PB_MODELrs> {
        MODE3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> MODE4_W<PB_MODELrs> {
        MODE4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> MODE5_W<PB_MODELrs> {
        MODE5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> MODE6_W<PB_MODELrs> {
        MODE6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> MODE7_W<PB_MODELrs> {
        MODE7_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Pin Mode Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_model::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_model::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_MODELrs;
impl crate::RegisterSpec for PB_MODELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_model::R`](R) reader structure"]
impl crate::Readable for PB_MODELrs {}
#[doc = "`write(|w| ..)` method takes [`pb_model::W`](W) writer structure"]
impl crate::Writable for PB_MODELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB_MODEL to value 0"]
impl crate::Resettable for PB_MODELrs {
    const RESET_VALUE: u32 = 0;
}
