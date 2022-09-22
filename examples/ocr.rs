extern crate mmdeploy;
extern crate opencv;

use std::env;

use opencv::{
    core::{Scalar, Point},
    types::{VectorOfi32, VectorOfPoint},
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{polylines},
};

pub use mmdeploy::text_detector::{
    text_detector_create_by_path,
    text_detector_apply,
    text_detector_release_result,
    text_detector_release,
};

pub use mmdeploy::text_recognizer::{
    text_recognizer_create_by_path,
    text_recognizer_apply_bbox,
    text_recognizer_release_result,
    text_recognizer_release,
};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let det_model_path = &args[2];
    let reg_model_path = &args[3];
    let image_path = &args[4];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let text_detector = text_detector_create_by_path(det_model_path, device_name, 0).unwrap();
    let text_recognizer = text_recognizer_create_by_path(reg_model_path, device_name, 0).unwrap();

    let text_det_result = text_detector_apply(text_detector, &img, 1).unwrap();

    let bbox_count = text_det_result.get_result_count(0);

    println!("bbox_count={}", bbox_count);

    let texts = text_recognizer_apply_bbox(text_recognizer, &img, 1, text_det_result.results, bbox_count).unwrap();

    for i in 0..bbox_count {
        let text = texts.get_result(0, i as isize);
        // let mask = det_result.get_result_mask(i)
        
        println!("box{}: {}", i, text.get_text());

        let mut poly_points = VectorOfPoint::new();

        let result = text_det_result.get_result(0, i as isize);

        for j in 0..4 {
            let pt = result.get_bbox(j as isize);
            format!("x: {}, y: {}, ", pt.x, pt.y);
            poly_points.push(Point{x: pt.x as i32, y: pt.y as i32});
            println!();
        }
        
        let color: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);

        polylines(&mut img, &poly_points, true, color, 1, 8, 0).unwrap();
    }

    let params = VectorOfi32::new();

    let succcess = imwrite("output_ocr.png", &img, &params).unwrap();

    text_detector_release_result(text_det_result, 1);
    text_detector_release(text_detector);
    text_recognizer_release_result(texts, 1);
    text_recognizer_release(text_recognizer);

}