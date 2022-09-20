extern crate mmdeploy;
extern crate opencv;

use std::env;

use opencv::{
    core::{Scalar, Point},
    types::VectorOfi32,
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{circle},
};

pub use mmdeploy::pose_detector::{pose_detector_create_by_path, pose_detector_apply, pose_detector_release_result, pose_detector_release};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let pose_detector = pose_detector_create_by_path(model_path, device_name, 0).unwrap();

    let pose_det_result = pose_detector_apply(pose_detector, &img, 1).unwrap();

    let result = pose_det_result.get_result(0, 0);

    let color: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);

    for i in 0..result.length {
        let point = result.get_point(i as isize);
        circle(&mut img, Point{x: point.x as i32, y: point.y as i32}, 1, color, 1, 8, 0).unwrap();
    }

    let params = VectorOfi32::new();

    let succcess = imwrite("output_pose.png", &img, &params).unwrap();

    pose_detector_release_result(pose_det_result, 1);
    pose_detector_release(pose_detector);

}