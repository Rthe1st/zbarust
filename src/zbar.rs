#![allow(clippy::missing_safety_doc)]

use enum_ordinalize::create_ordinalized_enum;
use libc::{c_int, c_uint};

create_ordinalized_enum!(pub ZBarColor,
    ZBarSpace = 0,
    ZBarBar = 1,
);

create_ordinalized_enum!(pub ZBarSymbolType,
    ZBarNone = 0,
    ZBarPartial = 1,
    ZBarEAN2 = 2,
    ZBarEAN5 = 5,
    ZBarEAN8 = 8,
    ZBarUPCE = 9,
    ZBarISBN10 = 10,
    ZBarUPCA = 12,
    ZBarEAN13 = 13,
    ZBarISBN13 = 14,
    ZBarComposite = 15,
    ZBarI25 = 25,
    ZBarDataBar = 34,
    ZBarDataBarExp = 35,
    ZBarCodeBar = 38,
    ZBarCode39 = 39,
    ZBarPDF417 = 57,
    ZBarQRCode = 64,
    ZBarCode93 = 93,
    ZBarCode128 = 128,
    ZBarSymbol = 0x00ff,
    ZBarAddOn2 = 0x0200,
    ZBarAddOn5 = 0x0500,
    ZBarAddOn = 0x0700,
);

create_ordinalized_enum!(pub ZBarOrientation,
    ZBarOrientUnknown = -1,
    ZBarOrientUp = 0,
    ZBarOrientRight = 1,
    ZBarOrientDown = 2,
    ZBarOrientLeft = 3
);

create_ordinalized_enum!(pub ZBarError,
    ZBarOK,
    ZBarErrNoMem,
    ZBarErrInternal,
    ZBarErrUnsupported,
    ZBarErrInvalid,
    ZBarErrSystem,
    ZBarErrLocking,
    ZBarErrBudy,
    ZBarErrXDisplay,
    ZBarErrXProto,
    ZBarErrClosed,
    ZBarErrWinAPI,
    ZBarErrNum
);

create_ordinalized_enum!(pub ZBarConfig,
    ZBarCfgEnable = 0,
    ZBarCfgAddCheck = 1,
    ZBarCfgEmitCheck = 2,
    ZBarCfgASCII = 3,
    ZBarCfgNum = 4,
    ZBarCfgMinLen = 0x20,
    ZBarCfgMaxLen = 0x21,
    ZBarCfgPosition = 0x80,
    ZBarCfgXDensity = 0x100,
    ZBarCfgYDensity = 0x101
);

create_ordinalized_enum!(pub ZBarModifier,
    ZBarModGS1,
    ZBarModAIM,
    ZBarModNum
);

create_ordinalized_enum!(pub VideoControlType,
    VideoCntlInteger = 1,
    VideoCntlMenu = 2,
    VideoCntlButton = 3,
    VideoCntlInteger64 = 4,
    VideoCntlString = 5,
    VideoCntlBoolean = 6,
);

extern {
    pub fn zbar_version(major: *mut c_uint, minor: *mut c_uint) -> c_int;
    pub fn zbar_set_verbosity(verbosity: c_int);
}
