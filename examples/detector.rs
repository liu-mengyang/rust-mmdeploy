extern crate mmdeploy;
extern crate opencv;

use std::env;

use mmdeploy::detector::detector_release;
use opencv::{
    core::{Scalar, Rect},
    types::VectorOfi32,
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{rectangle},
};

pub use mmdeploy::detector::{detector_create_by_path, detector_apply, detector_release_result};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let detector = detector_create_by_path(model_path, device_name, 0).unwrap();

    let det_result = detector_apply(detector, &img, 1).unwrap();

    let result_count = det_result.get_result_count(0);

    println!("bbox_count={}", result_count);

    for i in 0..result_count {
        let detection = det_result.get_result(0, i);
        // let mask = det_result.get_result_mask(i);
        println!("box {}, left={}, top={}, right={}, bottom={}, label={}, score={}",
            i,
            detection.bbox.left,
            detection.bbox.top,
            detection.bbox.right,
            detection.bbox.bottom,
            detection.label_id,
            detection.score);

        // skip detections with invalid bbox size (bbox height or width < 1)
        if (detection.bbox.right - detection.bbox.left) < 1.0 || (detection.bbox.bottom - detection.bbox.top) < 1.0 {
            continue;
        }

        // skip detections less than specified score threshold
        if detection.score < 0.3 {
            continue;
        }

        // generate mask overlay if model exports masks
        let width: i32 = (detection.bbox.right - detection.bbox.left) as i32;
        let height: i32 = (detection.bbox.bottom - detection.bbox.top) as i32;
        let rect: Rect = Rect {
            x: detection.bbox.left as i32,
            y: detection.bbox.top as i32,
            width: width,
            height: height,
        };
        let color: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);

        rectangle(&mut img, rect, color, 1, 8, 0).unwrap();
    }

    let params = VectorOfi32::new();

    let succcess = imwrite("output_detection.png", &img, &params).unwrap();

    detector_release_result(det_result, 1);
    detector_release(detector);

}