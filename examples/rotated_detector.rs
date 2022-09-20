extern crate mmdeploy;
extern crate opencv;

use std::env;

use opencv::{
    core::{Scalar, Point, no_array, Vector},
    types::{VectorOfi32, VectorOfPoint},
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{draw_contours},
};

pub use mmdeploy::rotated_detector::{rotated_detector_create_by_path, rotated_detector_apply, rotated_detector_release_result, rotated_detector_release};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let rotated_detector = rotated_detector_create_by_path(model_path, device_name, 0).unwrap();

    let rotated_det_result = rotated_detector_apply(rotated_detector, &img, 1).unwrap();

    let result_count = rotated_det_result.get_result_count(0);

    for i in 0..result_count {
        let detection = rotated_det_result.get_result(0, i);
        
        // skip low score
        if detection.score < 0.1 {
            continue;
        }

        let rbbox = detection.rbbox;
        let xc = rbbox[0];
        let yc = rbbox[1];
        let w = rbbox[2];
        let h = rbbox[3];
        let ag = rbbox[4];
        let wx = w / 2.0 * ag.cos();
        let wy = w / 2.0 * ag.sin();
        let hx = -h / 2.0 * ag.sin();
        let hy = h / 2.0 * ag.cos();

        let p1: Point = Point{x: (xc - wx - hx) as i32, y: (yc - wy - hy) as i32};
        let p2: Point = Point{x: (xc + wx - hx) as i32, y: (yc + wy - hy) as i32};
        let p3: Point = Point{x: (xc + wx + hx) as i32, y: (yc + wy + hy) as i32};
        let p4: Point = Point{x: (xc - wx + hx) as i32, y: (yc - wy + hy) as i32};

        let color: Scalar = Scalar::new(0.0, 255.0, 0.0, 0.0);

        let mut vecP = VectorOfPoint::new();
        vecP.push(p1);
        vecP.push(p2);
        vecP.push(p3);
        vecP.push(p4);

        let mut vecp: Vector<VectorOfPoint> = Vector::new();
        vecp.push(vecP);

        draw_contours(&mut img, &vecp, -1, color, 2, 8, &no_array(), i32::MAX, Point{x: 0, y:0});
    }


    let params = VectorOfi32::new();

    let succcess = imwrite("output_rotated_detection.png", &img, &params).unwrap();

    rotated_detector_release_result(rotated_det_result);
    rotated_detector_release(rotated_detector);

}