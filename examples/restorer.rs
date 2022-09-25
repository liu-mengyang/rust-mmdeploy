extern crate mmdeploy;
extern crate opencv;

use std::env;
use std::ffi::c_void;

use opencv::{
    core::{CV_8UC3, Mat, Scalar},
    types::VectorOfi32,
    imgcodecs::{imread,IMREAD_COLOR,imwrite},
    imgproc::{cvt_color, COLOR_RGB2BGR},
};

pub use mmdeploy::restorer::{restorer_create_by_path, restorer_apply, restorer_release_result, restorer_release};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let img = imread(image_path, IMREAD_COLOR).unwrap();

    let restorer = restorer_create_by_path(model_path, device_name, 0).unwrap();

    let res_result = restorer_apply(restorer, &img, 1).unwrap();

    let res = res_result.get_result(0);
    
    unsafe {
        let sr_img = match Mat::new_rows_cols_with_data(res.height, res.width, CV_8UC3, res.data as *mut c_void, 0) {
            Ok(mat) => mat,
            Err(error) => panic!("{}", error),
        };
            
        let color: Scalar = Scalar::new(0.0, 0.0, 0.0, 0.0);

        let mut res_img = Mat::new_rows_cols_with_default(res.height, res.width, CV_8UC3, color).unwrap();

        let _cvt_success = cvt_color(&sr_img, &mut res_img, COLOR_RGB2BGR, 3).unwrap();

        let params = VectorOfi32::new();

        let _succcess = imwrite("output_restorer.png", &res_img, &params).unwrap();

    }

    restorer_release_result(res_result, 1);
    restorer_release(restorer);
}