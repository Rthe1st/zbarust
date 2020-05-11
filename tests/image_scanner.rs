extern crate qrcode_generator;

use qrcode_generator::QrCodeEcc;

extern crate zbarust;
use zbarust::zbar::image::{ZBAR_NONE, ZBAR_QRCODE};
use zbarust::zbar::img_scanner::{
    zbar_image_scanner_create, zbar_image_scanner_destroy, zbar_image_scanner_s,
    zbar_image_scanner_set_config,
};

use zbarust::scanner;

use zbarust::zbar::config::ZBAR_CFG_ENABLE;

#[test]
fn image_create_destroy() {
    let scanner = unsafe { zbar_image_scanner_create() };

    unsafe {
        zbar_image_scanner_destroy(scanner);
    }
}

#[test]
fn set_config() {
    let scanner = unsafe { zbar_image_scanner_create() };

    let result = unsafe { zbar_image_scanner_set_config(scanner, ZBAR_NONE, ZBAR_CFG_ENABLE, 0) };
    if result != 0 {
        assert!(false);
    }

    let result = unsafe { zbar_image_scanner_set_config(scanner, ZBAR_QRCODE, ZBAR_CFG_ENABLE, 0) };
    if result != 0 {
        assert!(false);
    }
}

#[test]
fn decode_qrcode() {
    let scanner = unsafe { zbar_image_scanner_create() };

    let url = "https://magiclen.org";

    let size = 512;

    let mut data = qrcode_generator::to_image(url, QrCodeEcc::Low, size).unwrap();

    //need to recode scan_y800 next
    let mut result;
    unsafe {
        result = scanner::scan_y800(scanner, &mut data, size as u32, size as u32).unwrap();
    }

    assert_eq!(1, result.len());
    assert_eq!(ZBAR_QRCODE, result[0].symbol_type);
    assert_eq!(url, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}
