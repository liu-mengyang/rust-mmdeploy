use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;


pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn from(rect: mmdeploy_rect_t) -> Rect {
        Rect {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

pub struct BBox {
    pub bbox: Rect,
    pub label_id: i32,
    pub score: f32,
    pub mask: *mut mmdeploy_instance_mask_t,
}

impl BBox {
    pub fn from(detection: mmdeploy_detection_t) -> BBox {
        BBox {
            bbox: Rect::from(detection.bbox),
            label_id: detection.label_id as i32,
            score: detection.score,
            mask: detection.mask,
        }
    }
}

pub struct DetResult {
    pub results: *mut *mut mmdeploy_detection_t,
    pub result_count: *mut *mut i32,
}

impl DetResult {
    pub fn new() -> DetResult {
        DetResult {
            results: Box::into_raw(Box::new(mmdeploy_detection_t::new())) as *mut *mut mmdeploy_detection_t,
            result_count: Box::into_raw(Box::new(0)) as *mut *mut i32,
        }
    }

    pub fn get_result_count(&self, image_index:isize) -> isize {
        unsafe{
            *(*self.result_count.offset(image_index)) as isize
        }
    }


    pub fn get_result(&self, image_index:isize, box_index: isize) -> BBox {
        unsafe{
            BBox::from(*(*self.results.offset(image_index)).offset(box_index))
        }
    }
}

pub fn detector_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_detector_t, mmdeploy_status_t> {
    unsafe{
        let mut detector: mmdeploy_detector_t = &mut mmdeploy_detector::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_detector_create_by_path(model_path_i8, device_name_i8, device_id, &mut detector).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(detector)
    }
}


pub fn detector_apply(detector: mmdeploy_detector_t, img: &Mat, mat_count: i32) -> Result<DetResult, mmdeploy_status_t> {
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

        let det_result = DetResult::new();

        let status: mmdeploy_status_t = mmdeploy_detector_apply(detector, mat, mat_count, det_result.results, det_result.result_count).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(det_result)
    }
}

pub fn detector_release_result(det_result: DetResult, image_num: i32) {
    unsafe { mmdeploy_detector_release_result(*det_result.results, *det_result.result_count, image_num); }
}

pub fn detector_release(detector: mmdeploy_detector_t) {
    unsafe { mmdeploy_detector_destroy(detector); }
}