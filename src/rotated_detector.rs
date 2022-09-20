use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;


pub struct RBBox {
    pub label_id: i32,
    pub score: f32,
    pub rbbox: [f32; 5usize],
}

impl RBBox {
    pub fn from(rotated_detection: mmdeploy_rotated_detection_t) -> RBBox {
        RBBox {
            label_id: rotated_detection.label_id as i32,
            score: rotated_detection.score,
            rbbox: rotated_detection.rbbox,
        }
    }
}

pub struct RotResult {
    pub results: *mut *mut mmdeploy_rotated_detection_t,
    pub result_count: *mut *mut i32,
}

impl RotResult {
    pub fn new() -> RotResult {
        RotResult {
            results: Box::into_raw(Box::new(mmdeploy_rotated_detection_t::new())) as *mut *mut mmdeploy_rotated_detection_t,
            result_count: Box::into_raw(Box::new(0)) as *mut *mut i32,
        }
    }

    pub fn get_result_count(&self, image_index:isize) -> isize {
        unsafe{
            *(*self.result_count.offset(image_index)) as isize
        }
    }


    pub fn get_result(&self, image_index:isize, box_index: isize) -> RBBox {
        unsafe{
            RBBox::from(*(*self.results.offset(image_index)).offset(box_index))
        }
    }
}

pub fn rotated_detector_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_rotated_detector_t, mmdeploy_status_t> {
    unsafe{
        let mut rotated_detector: mmdeploy_rotated_detector_t = &mut mmdeploy_rotated_detector::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_rotated_detector_create_by_path(model_path_i8, device_name_i8, device_id, &mut rotated_detector).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(rotated_detector)
    }
}


pub fn rotated_detector_apply(rotated_detector: mmdeploy_rotated_detector_t, img: &Mat, mat_count: i32) -> Result<RotResult, mmdeploy_status_t> {
    unsafe {
        let mat: *const mmdeploy_mat_t = &mmdeploy_mat_t {
            data: img.data() as *mut u8,
            height: img.rows(),
            width: img.cols(),
            channel: 3,
            format: mmdeploy_pixel_format_t_MMDEPLOY_PIXEL_FORMAT_BGR,
            type_: mmdeploy_data_type_t_MMDEPLOY_DATA_TYPE_UINT8,
        };

        let mut rotResult = RotResult::new();

        let status: mmdeploy_status_t = mmdeploy_rotated_detector_apply(rotated_detector, mat, mat_count, rotResult.results, rotResult.result_count).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(rotResult)
    }
}

pub fn rotated_detector_release_result(rotated_det_result: RotResult) {
    unsafe { mmdeploy_rotated_detector_release_result(*rotated_det_result.results, *rotated_det_result.result_count); }
}

pub fn rotated_detector_release(rotated_detector: mmdeploy_rotated_detector_t) {
    unsafe { mmdeploy_rotated_detector_destroy(rotated_detector); }
}