extern crate mmdeploy;
extern crate opencv;
extern crate rand;

use std::env;

use opencv::{
    core::{CV_8UC3, MatExpr, Vector, Vec3b},
    prelude::*,
    types::VectorOfi32,
    imgcodecs::{imread, imwrite, IMREAD_COLOR},
    imgproc::{rectangle},
};

pub use mmdeploy::segmentor::{segmentor_create_by_path, segmentor_apply, segmentor_release_result, segmentor_release};
use rand::Rng;


fn gen_palette(num_classes: i32) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();

    let mut palette = Vec::new();

    for i in 0..num_classes {
        let r = rng.gen_range(0..255) as u8;
        let g = rng.gen_range(0..255) as u8;
        let b = rng.gen_range(0..255) as u8;

        let mut color = Vec::new();
        color.push(r);
        color.push(g);
        color.push(b);
        
        palette.push(color);
    }
    palette

}

fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let segmentor = segmentor_create_by_path(model_path, device_name, 0).unwrap();

    let seg_result = segmentor_apply(segmentor, &img, 1).unwrap();
    
    let result = seg_result.get_result(0, 0);

    let palette = gen_palette(result.classes + 1);
    
    let mut color_mask = Mat::zeros(result.height, result.width, CV_8UC3).unwrap();



    let pos:i32 = 0;

    let mut color_mat = Box::new(color_mask.to_mat().unwrap());

    let test_mask = color_mat.at_2d_mut::<Vec3b>(511, 1023).unwrap();

    for row in 0..color_mat.rows() {
        for col in 0..color_mat.cols() {
            let colors = palette[result.getMask(row * color_mat.cols()+col) as usize].clone();
            let mut mask = color_mat.at_2d_mut::<Vec3b>(row, col).unwrap();
            mask[0] += colors[0];
            mask[1] += colors[1];
            mask[2] += colors[2];
        }
    }

    let test_mask = color_mat.at_2d_mut::<Vec3b>(511, 1023).unwrap();
    
    let img = match (img * 0.5 + *color_mat * 0.5).into_result() {
        Ok(matexpr) => matexpr,
        Err(error) => panic!("{}", error),
    };

    let params = VectorOfi32::new();

    let succcess = imwrite("output_segmentation.png", &img, &params).unwrap();

    segmentor_release_result(seg_result, 1);
    segmentor_release(segmentor);

}