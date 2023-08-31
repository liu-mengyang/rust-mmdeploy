use std::os::raw::c_char;
use std::ffi::{CString, CStr};

use sys::*;
use opencv::prelude::*;

pub struct Text {
    pub text: *mut c_char,
    pub score: *mut f32,
    pub length: i32,
}

impl Text {
    pub fn from(recognition: mmdeploy_text_recognition_t) -> Text {
        Text {
            text: recognition.text,
            score: recognition.score,
            length: recognition.length,
        }
    }
    pub fn get_text(&self) -> String {
        unsafe {
            let text_slice: String = String::from(CString::from(CStr::from_ptr(self.text)).to_str().unwrap());
            text_slice
        }
    }
}

pub struct RecResult {
    pub results: *mut *mut mmdeploy_text_recognition_t,
}

impl RecResult {
    pub fn new() -> RecResult {
        RecResult {
            results: Box::into_raw(Box::new(mmdeploy_text_recognition_t::new())) as *mut *mut mmdeploy_text_recognition_t,
        }
    }

    pub fn get_result(&self, image_index:isize, text_index: isize) -> Text {
        unsafe{
            Text::from(*(*self.results.offset(image_index)).offset(text_index))
        }
    }
}

pub fn text_recognizer_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_text_recognizer_t, mmdeploy_status_t> {
    unsafe{
        let mut recognizer: mmdeploy_text_recognizer_t = &mut mmdeploy_text_recognizer::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_text_recognizer_create_by_path(model_path_i8, device_name_i8, device_id, &mut recognizer).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(recognizer)
    }
}


pub fn text_recognizer_apply_bbox(text_recognizer: mmdeploy_text_recognizer_t, img: &Mat, mat_count: i32, bboxes: *mut *mut mmdeploy_text_detection_t, bbox_count: i32) -> Result<RecResult, mmdeploy_status_t> {
    unsafe {
        let mat: *const mmdeploy_mat_t = &mmdeploy_mat_t {
            data: img.data() as *mut u8,
            height: img.rows(),
            width: img.cols(),
            channel: 3,
            format: mmdeploy_pixel_format_t_MMDEPLOY_PIXEL_FORMAT_BGR,
            type_: mmdeploy_data_type_t_MMDEPLOY_DATA_TYPE_UINT8,
            device: std::ptr::null_mut(),
        };

        let rec_result = RecResult::new();

        let status: mmdeploy_status_t = mmdeploy_text_recognizer_apply_bbox(text_recognizer, mat, mat_count, *bboxes as *const mmdeploy_text_detection_t, &bbox_count, rec_result.results).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(rec_result)
    }
}

pub fn text_recognizer_release_result(rec_result: RecResult, image_num: i32) {
    unsafe { mmdeploy_text_recognizer_release_result(*rec_result.results, image_num); }
}

pub fn text_recognizer_release(text_recognizer: mmdeploy_text_recognizer_t) {
    unsafe { mmdeploy_text_recognizer_destroy(text_recognizer); }
}