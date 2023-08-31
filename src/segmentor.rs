use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;

pub struct Seg {
    pub height: i32,
    pub width: i32,
    pub classes: i32,
    pub mask: *mut i32,
}

impl Seg {
    pub fn from(segmentation: mmdeploy_segmentation_t) -> Seg {
        Seg {
            height: segmentation.height,
            width: segmentation.width,
            classes: segmentation.classes,
            mask: segmentation.mask,
        }
    }

    pub fn get_mask(&self, index: i32) -> i32{
        unsafe { *self.mask.offset(index as isize) }
    }
}

pub struct SegResult {
    pub results: *mut *mut mmdeploy_segmentation_t,
}

impl SegResult {
    pub fn new() -> SegResult {
        SegResult {
            results: Box::into_raw(Box::new(mmdeploy_segmentation_t::new())) as *mut *mut mmdeploy_segmentation_t,
        }
    }

    pub fn get_result(&self, image_index:isize, seg_index: isize) -> Seg {
        unsafe{
            Seg::from(*(*self.results.offset(image_index)).offset(seg_index))
        }
    }
}

pub fn segmentor_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_segmentor_t, mmdeploy_status_t> {
    unsafe{
        let mut segmentor: mmdeploy_segmentor_t = &mut mmdeploy_segmentor::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_segmentor_create_by_path(model_path_i8, device_name_i8, device_id, &mut segmentor).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(segmentor)
    }
}


pub fn segmentor_apply(segmentor: mmdeploy_segmentor_t, img: &Mat, mat_count: i32) -> Result<SegResult, mmdeploy_status_t> {
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

        let seg_result = SegResult::new();

        let status: mmdeploy_status_t = mmdeploy_segmentor_apply(segmentor, mat, mat_count, seg_result.results).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(seg_result)
    }
}

pub fn segmentor_release_result(seg_result: SegResult, image_num: i32) {
    unsafe { mmdeploy_segmentor_release_result(*seg_result.results, image_num); }
}

pub fn segmentor_release(segmentor: mmdeploy_segmentor_t) {
    unsafe { mmdeploy_segmentor_destroy(segmentor); }
}