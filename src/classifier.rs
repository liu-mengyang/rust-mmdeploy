use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;

pub struct Cls {
    pub label_id: i32,
    pub score: f32,
}

impl Cls {
    pub fn from(classification: mmdeploy_classification_t) -> Cls {
        Cls {
            label_id: classification.label_id as i32,
            score: classification.score,
        }
    }
}

pub struct ClsResult {
    pub results: *mut *mut mmdeploy_classification_t,
    pub result_count: *mut *mut i32,
}

impl ClsResult {
    pub fn new() -> ClsResult {
        ClsResult {
            results: Box::into_raw(Box::new(mmdeploy_classification_t::new())) as *mut *mut mmdeploy_classification_t,
            result_count: Box::into_raw(Box::new(0)) as *mut *mut i32,
        }
    }

    pub fn get_result_count(&self, image_index:isize) -> isize {
        unsafe{
            *(*self.result_count.offset(image_index)) as isize
        }
    }


    pub fn get_result(&self, image_index:isize, cls_index: isize) -> Cls {
        unsafe{
            Cls::from(*(*self.results.offset(image_index)).offset(cls_index))
        }
    }
}

pub fn classifier_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_classifier_t, mmdeploy_status_t> {
    unsafe{
        let mut classifier: mmdeploy_classifier_t = &mut mmdeploy_classifier::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_classifier_create_by_path(model_path_i8, device_name_i8, device_id, &mut classifier).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(classifier)
    }
}


pub fn classifier_apply(classifier: mmdeploy_classifier_t, img: &Mat, mat_count: i32) -> Result<ClsResult, mmdeploy_status_t> {
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

        let cls_result = ClsResult::new();

        let status: mmdeploy_status_t = mmdeploy_classifier_apply(classifier, mat, mat_count, cls_result.results, cls_result.result_count).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(cls_result)
    }
}

pub fn classifier_release_result(cls_result: ClsResult, image_num: i32) {
    unsafe { mmdeploy_classifier_release_result(*cls_result.results, *cls_result.result_count, image_num); }
}

pub fn classifier_release(classifier: mmdeploy_classifier_t) {
    unsafe { mmdeploy_classifier_destroy(classifier); }
}