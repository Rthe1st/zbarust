extern crate zbarust;

use zbarust::zbar::image::{
    zbar_image_create, zbar_image_destroy
};

#[test]
fn image_create_destroy() {
    let image = unsafe{ zbar_image_create() };
    unsafe{ zbar_image_destroy(image) };

}
