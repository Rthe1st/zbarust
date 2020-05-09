#![allow(clippy::missing_safety_doc)]

use libc::{c_int, c_uint};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarColor {
    Space = 0,
    Bar = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarSymbolType {
    None = 0,
    Partial = 1,
    Ean2 = 2,
    Ean5 = 5,
    Ean8 = 8,
    Upce = 9,
    Isbn10 = 10,
    Upca = 12,
    Ean13 = 13,
    Isbn13 = 14,
    Composite = 15,
    I25 = 25,
    Databar = 34,
    DatabarExp = 35,
    Codabar = 38,
    Code39 = 39,
    Pdf417 = 57,
    Qrcode = 64,
    Code93 = 93,
    Code128 = 128,
    Symbol = 255,
    Addon2 = 512,
    Addon5 = 1280,
    Addon = 1792,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarOrientation {
    OrientUnknown = -1,
    OrientUp = 0,
    OrientRight = 1,
    OrientDown = 2,
    OrientLeft = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarError {
    Ok = 0,
    ErrNomem = 1,
    ErrInternal = 2,
    ErrUnsupported = 3,
    ErrInvalid = 4,
    ErrSystem = 5,
    ErrLocking = 6,
    ErrBusy = 7,
    ErrXdisplay = 8,
    ErrXproto = 9,
    ErrClosed = 10,
    ErrWinapi = 11,
    ErrNum = 12,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarConfig {
    CfgEnable = 0,
    CfgAddCheck = 1,
    CfgEmitCheck = 2,
    CfgAscii = 3,
    CfgNum = 4,
    CfgMinLen = 32,
    CfgMaxLen = 33,
    CfgUncertainty = 64,
    CfgPosition = 128,
    CfgXDensity = 256,
    CfgYDensity = 257,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ZBarModifier {
    ModGs1 = 0,
    ModAim = 1,
    ModNum = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VideoControlType {
    VideoCntlInteger = 1,
    VideoCntlMenu = 2,
    VideoCntlButton = 3,
    VideoCntlInteger64 = 4,
    VideoCntlString = 5,
    VideoCntlBoolean = 6,
}

extern {
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_int;
    pub fn zbar_set_verbosity(verbosity: c_int);
}
