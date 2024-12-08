///Register `MODEL` reader
pub type R = crate::R<MODELrs>;
///Register `MODEL` writer
pub type W = crate::W<MODELrs>;
///Pin 0 Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0 {
    ///0: Input disabled. Pullup if DOUT is set.
    Disabled = 0,
    ///1: Input enabled. Filter if DOUT is set
    Input = 1,
    ///2: Input enabled. DOUT determines pull direction
    Inputpull = 2,
    ///3: Input enabled with filter. DOUT determines pull direction
    Inputpullfilter = 3,
    ///4: Push-pull output
    Pushpull = 4,
    ///5: Push-pull using alternate control
    Pushpullalt = 5,
    ///6: Wired-or output
    Wiredor = 6,
    ///7: Wired-or output with pull-down
    Wiredorpulldown = 7,
    ///8: Open-drain output
    Wiredand = 8,
    ///9: Open-drain output with filter
    Wiredandfilter = 9,
    ///10: Open-drain output with pullup
    Wiredandpullup = 10,
    ///11: Open-drain output with filter and pullup
    Wiredandpullupfilter = 11,
    ///12: Open-drain output using alternate control
    Wiredandalt = 12,
    ///13: Open-drain output using alternate control with filter
    Wiredandaltfilter = 13,
    ///14: Open-drain output using alternate control with pullup
    Wiredandaltpullup = 14,
    ///15: Open-drain output using alternate control with filter and pullup
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
impl crate::IsEnum for MODE0 {}
///Field `MODE0` reader - Pin 0 Mode
pub type Mode0R = crate::FieldReader<MODE0>;
impl Mode0R {
    ///Get enumerated values variant
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
    ///Input disabled. Pullup if DOUT is set.
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0::Disabled
    }
    ///Input enabled. Filter if DOUT is set
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0::Input
    }
    ///Input enabled. DOUT determines pull direction
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0::Inputpull
    }
    ///Input enabled with filter. DOUT determines pull direction
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0::Inputpullfilter
    }
    ///Push-pull output
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0::Pushpull
    }
    ///Push-pull using alternate control
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE0::Pushpullalt
    }
    ///Wired-or output
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0::Wiredor
    }
    ///Wired-or output with pull-down
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0::Wiredorpulldown
    }
    ///Open-drain output
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0::Wiredand
    }
    ///Open-drain output with filter
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0::Wiredandfilter
    }
    ///Open-drain output with pullup
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0::Wiredandpullup
    }
    ///Open-drain output with filter and pullup
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0::Wiredandpullupfilter
    }
    ///Open-drain output using alternate control
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE0::Wiredandalt
    }
    ///Open-drain output using alternate control with filter
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE0::Wiredandaltfilter
    }
    ///Open-drain output using alternate control with pullup
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE0::Wiredandaltpullup
    }
    ///Open-drain output using alternate control with filter and pullup
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE0::Wiredandaltpullupfilter
    }
}
///Field `MODE0` writer - Pin 0 Mode
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE0, crate::Safe>;
impl<'a, REG> Mode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input disabled. Pullup if DOUT is set.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Disabled)
    }
    ///Input enabled. Filter if DOUT is set
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Input)
    }
    ///Input enabled. DOUT determines pull direction
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Inputpull)
    }
    ///Input enabled with filter. DOUT determines pull direction
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Inputpullfilter)
    }
    ///Push-pull output
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Pushpull)
    }
    ///Push-pull using alternate control
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Pushpullalt)
    }
    ///Wired-or output
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredor)
    }
    ///Wired-or output with pull-down
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredorpulldown)
    }
    ///Open-drain output
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredand)
    }
    ///Open-drain output with filter
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandfilter)
    }
    ///Open-drain output with pullup
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandpullup)
    }
    ///Open-drain output with filter and pullup
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandpullupfilter)
    }
    ///Open-drain output using alternate control
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandalt)
    }
    ///Open-drain output using alternate control with filter
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltfilter)
    }
    ///Open-drain output using alternate control with pullup
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltpullup)
    }
    ///Open-drain output using alternate control with filter and pullup
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0::Wiredandaltpullupfilter)
    }
}
///Field `MODE1` reader - Pin 1 Mode
pub use Mode0R as Mode1R;
///Field `MODE2` reader - Pin 2 Mode
pub use Mode0R as Mode2R;
///Field `MODE3` reader - Pin 3 Mode
pub use Mode0R as Mode3R;
///Field `MODE4` reader - Pin 4 Mode
pub use Mode0R as Mode4R;
///Field `MODE5` reader - Pin 5 Mode
pub use Mode0R as Mode5R;
///Field `MODE6` reader - Pin 6 Mode
pub use Mode0R as Mode6R;
///Field `MODE7` reader - Pin 7 Mode
pub use Mode0R as Mode7R;
///Field `MODE1` writer - Pin 1 Mode
pub use Mode0W as Mode1W;
///Field `MODE2` writer - Pin 2 Mode
pub use Mode0W as Mode2W;
///Field `MODE3` writer - Pin 3 Mode
pub use Mode0W as Mode3W;
///Field `MODE4` writer - Pin 4 Mode
pub use Mode0W as Mode4W;
///Field `MODE5` writer - Pin 5 Mode
pub use Mode0W as Mode5W;
///Field `MODE6` writer - Pin 6 Mode
pub use Mode0W as Mode6W;
///Field `MODE7` writer - Pin 7 Mode
pub use Mode0W as Mode7W;
///Pin 1 Mode
pub use MODE0 as MODE1;
///Pin 2 Mode
pub use MODE0 as MODE2;
///Pin 3 Mode
pub use MODE0 as MODE3;
///Pin 4 Mode
pub use MODE0 as MODE4;
///Pin 5 Mode
pub use MODE0 as MODE5;
///Pin 6 Mode
pub use MODE0 as MODE6;
///Pin 7 Mode
pub use MODE0 as MODE7;
impl R {
    ///Bits 0:3 - Pin 0 Mode
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Pin 1 Mode
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Pin 2 Mode
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Pin 3 Mode
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Pin 4 Mode
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Pin 5 Mode
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Pin 6 Mode
    #[inline(always)]
    pub fn mode6(&self) -> Mode6R {
        Mode6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Pin 7 Mode
    #[inline(always)]
    pub fn mode7(&self) -> Mode7R {
        Mode7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEL")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Pin 0 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode0(&mut self) -> Mode0W<MODELrs> {
        Mode0W::new(self, 0)
    }
    ///Bits 4:7 - Pin 1 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> Mode1W<MODELrs> {
        Mode1W::new(self, 4)
    }
    ///Bits 8:11 - Pin 2 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> Mode2W<MODELrs> {
        Mode2W::new(self, 8)
    }
    ///Bits 12:15 - Pin 3 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode3(&mut self) -> Mode3W<MODELrs> {
        Mode3W::new(self, 12)
    }
    ///Bits 16:19 - Pin 4 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode4(&mut self) -> Mode4W<MODELrs> {
        Mode4W::new(self, 16)
    }
    ///Bits 20:23 - Pin 5 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode5(&mut self) -> Mode5W<MODELrs> {
        Mode5W::new(self, 20)
    }
    ///Bits 24:27 - Pin 6 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode6(&mut self) -> Mode6W<MODELrs> {
        Mode6W::new(self, 24)
    }
    ///Bits 28:31 - Pin 7 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode7(&mut self) -> Mode7W<MODELrs> {
        Mode7W::new(self, 28)
    }
}
///Port Pin Mode Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`model::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`model::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MODELrs;
impl crate::RegisterSpec for MODELrs {
    type Ux = u32;
}
///`read()` method returns [`model::R`](R) reader structure
impl crate::Readable for MODELrs {}
///`write(|w| ..)` method takes [`model::W`](W) writer structure
impl crate::Writable for MODELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODEL to value 0
impl crate::Resettable for MODELrs {
    const RESET_VALUE: u32 = 0;
}
