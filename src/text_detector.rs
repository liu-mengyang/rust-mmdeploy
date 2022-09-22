use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;


pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from(point: mmdeploy_point_t) -> Point {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}

pub struct TextDet {
    pub bbox: [mmdeploy_point_t; 4usize],
    pub score: f32,
}

impl TextDet {
    pub fn from(text_det_detection: mmdeploy_text_detection_t) -> TextDet {
        TextDet {
            bbox: text_det_detection.bbox,
            score: text_det_detection.score,
        }
    }
    
    pub fn get_bbox(&self, point_index: isize) -> Point {
        unsafe {Point::from(self.bbox[point_index as usize])}
    }
}

pub struct TextDetResult {
    pub results: *mut *mut mmdeploy_text_detection_t,
    pub result_count: *mut *mut i32,
}

impl TextDetResult {
    pub fn new() -> TextDetResult {
        TextDetResult {
            results: Box::into_raw(Box::new(mmdeploy_text_detection_t::new())) as *mut *mut mmdeploy_text_detection_t,
            result_count: Box::into_raw(Box::new(0)) as *mut *mut i32,
        }
    }

    pub fn get_result(&self, image_index:isize, box_index: isize) -> TextDet {
        unsafe{
            TextDet::from(*(*self.results.offset(image_index)).offset(box_index))
        }
    }

    pub fn get_result_count(&self, image_index: isize) -> i32 {
        unsafe{
            *(*self.result_count.offset(image_index)) as i32
        }
    }
}

pub fn text_detector_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_text_detector_t, mmdeploy_status_t> {
    unsafe{
        let mut text_detector: mmdeploy_text_detector_t = &mut mmdeploy_text_detector::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_text_detector_create_by_path(model_path_i8, device_name_i8, device_id, &mut text_detector).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(text_detector)
    }
}


pub fn text_detector_apply(text_detector: mmdeploy_text_detector_t, img: &Mat, mat_count: i32) -> Result<TextDetResult, mmdeploy_status_t> {
    unsafe {
        let mat: *const mmdeploy_mat_t = &mmdeploy_mat_t {
            data: img.data() as *mut u8,
            height: img.rows(),
            width: img.cols(),
            channel: 3,
            format: mmdeploy_pixel_format_t_MMDEPLOY_PIXEL_FORMAT_BGR,
            type_: mmdeploy_data_type_t_MMDEPLOY_DATA_TYPE_UINT8,
        };

        let mut textDetResult = TextDetResult::new();

        println!("apply");
        let status: mmdeploy_status_t = mmdeploy_text_detector_apply(text_detector, mat, mat_count, textDetResult.results, textDetResult.result_count).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(textDetResult)
    }
}

pub fn text_detector_release_result(text_det_result: TextDetResult, image_num: i32) {
    unsafe { mmdeploy_text_detector_release_result(*text_det_result.results, *text_det_result.result_count, image_num); }
}

pub fn text_detector_release(text_detector: mmdeploy_text_detector_t) {
    unsafe { mmdeploy_text_detector_destroy(text_detector); }
}