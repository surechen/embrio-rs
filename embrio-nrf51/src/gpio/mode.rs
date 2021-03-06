use nrf51::gpio::pin_cnf;

pub trait InputMode: Sized {
    fn apply(w: &mut pin_cnf::W) -> Self;
}

pub trait OutputMode: Sized {
    fn apply(w: &mut pin_cnf::W) -> Self;
}

pub trait PinMode: Sized {
    fn apply(w: &mut pin_cnf::W) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Floating {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct PullUp {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct PullDown {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct PushPull {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct OpenDrain {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct Unconfigured {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct Disabled {
    _reserved: (),
}

#[derive(Debug, Copy, Clone)]
pub struct Input<Mode> {
    mode: Mode,
}

#[derive(Debug, Copy, Clone)]
pub struct Output<Mode> {
    mode: Mode,
}

impl InputMode for Floating {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.pull().disabled();
        Floating { _reserved: () }
    }
}

impl InputMode for PullUp {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.pull().pullup();
        PullUp { _reserved: () }
    }
}

impl InputMode for PullDown {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.pull().pulldown();
        PullDown { _reserved: () }
    }
}

impl OutputMode for PushPull {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.drive().s0s1();
        PushPull { _reserved: () }
    }
}

impl OutputMode for OpenDrain {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.drive().s0d1();
        OpenDrain { _reserved: () }
    }
}

impl PinMode for Disabled {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.dir().input().input().disconnect();
        Disabled { _reserved: () }
    }
}

impl<Mode: InputMode> PinMode for Input<Mode> {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.dir().input().input().connect();
        Input {
            mode: Mode::apply(w),
        }
    }
}

impl<Mode: OutputMode> PinMode for Output<Mode> {
    #[inline(always)]
    fn apply(w: &mut pin_cnf::W) -> Self {
        w.dir().output();
        Output {
            mode: Mode::apply(w),
        }
    }
}

impl Unconfigured {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Unconfigured { _reserved: () }
    }
}

impl Input<Unconfigured> {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Input {
            mode: Unconfigured { _reserved: () },
        }
    }
}

impl Output<Unconfigured> {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Output {
            mode: Unconfigured { _reserved: () },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::mem;

    // TODO: Static assert with const size_of fn?
    #[test]
    fn zst() {
        assert!(mem::size_of::<Input<Unconfigured>>() == 0);
        assert!(mem::size_of::<Input<Floating>>() == 0);
        assert!(mem::size_of::<Input<PullUp>>() == 0);
        assert!(mem::size_of::<Input<PullDown>>() == 0);
        assert!(mem::size_of::<Output<Unconfigured>>() == 0);
        assert!(mem::size_of::<Output<PushPull>>() == 0);
        assert!(mem::size_of::<Output<OpenDrain>>() == 0);
        assert!(mem::size_of::<Disabled>() == 0);
        assert!(mem::size_of::<Unconfigured>() == 0);
    }
}
