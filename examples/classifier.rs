extern crate mmdeploy;
extern crate opencv;

use std::env;

use opencv::{
    imgcodecs::{imread,IMREAD_COLOR},
};

pub use mmdeploy::classifier::{classifier_create_by_path, classifier_apply, classifier_release_result, classifier_release};


fn main() {
    let args: Vec<String> = env::args().collect();

    let device_name = &args[1];
    let model_path = &args[2];
    let image_path = &args[3];

    // opencv
    let mut img = imread(image_path, IMREAD_COLOR).unwrap();

    let classifier = classifier_create_by_path(model_path, device_name, 0).unwrap();

    let cls_result = classifier_apply(classifier, &img, 1).unwrap();

    let result_count = cls_result.get_result_count(0);

    for i in 0..result_count {
        let classification = cls_result.get_result(0, i);
        
        println!("label={}, score={}",
            classification.label_id,
            classification.score);
    }

    classifier_release_result(cls_result, 1);
    classifier_release(classifier);
}