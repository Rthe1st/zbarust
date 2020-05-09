extern crate qrcode_generator;
extern crate zbarust;

use zbarust::ZBarConfig;
use zbarust::ZBarImageScanner;
use zbarust::ZBarSymbolType;

use qrcode_generator::QrCodeEcc;

#[test]
fn image_create_destroy() {
    let _scanner = ZBarImageScanner::new();
}

#[test]
fn set_config() {
    let scanner = ZBarImageScanner::new();
    unsafe {
        (*scanner).set_config(ZBarSymbolType::None, ZBarConfig::CfgEnable, 0).unwrap();
        (*scanner).set_config(ZBarSymbolType::Qrcode, ZBarConfig::CfgEnable, 1).unwrap();
    }
}

#[test]
fn decode_qrcode() {
    let scanner = ZBarImageScanner::new();

    let url = "https://magiclen.org";

    let size = 512;

    let mut data = qrcode_generator::to_image(url, QrCodeEcc::Low, size).unwrap();

    let mut result;
    unsafe {
        result = (*scanner).scan_y800(&mut data, size as u32, size as u32).unwrap();
    }

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::Qrcode, result[0].symbol_type);
    assert_eq!(url, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}
