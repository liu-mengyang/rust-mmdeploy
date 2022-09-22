use std::os::raw::c_char;
use std::ffi::CString;

use sys::*;
use opencv::prelude::*;

pub struct MMat {
    pub data: *mut u8,
    pub height: i32,
    pub width: i32,
    pub channel: i32,
    pub format: mmdeploy_pixel_format_t,
    pub type_: mmdeploy_data_type_t,
}

impl MMat {
    pub fn from(mat: mmdeploy_mat_t) -> MMat {
        MMat {
            data: mat.data,
            height: mat.height,
            width: mat.width,
            channel: mat.channel,
            format: mat.format,
            type_: mat.type_,
        }
    }
}

pub struct ResResult {
    pub results: *mut *mut mmdeploy_mat_t,
}

impl ResResult {
    pub fn new() -> ResResult {
        ResResult {
            results: Box::into_raw(Box::new(mmdeploy_mat_t::new())) as *mut *mut mmdeploy_mat_t,
        }
    }

    pub fn get_result(&self, image_index:isize) -> MMat {
        unsafe{
            MMat::from(*(*self.results.offset(image_index)))
        }
    }
}

pub fn restorer_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_restorer_t, mmdeploy_status_t> {
    unsafe{
        let mut restorer: mmdeploy_restorer_t = &mut mmdeploy_restorer::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_restorer_create_by_path(model_path_i8, device_name_i8, device_id, &mut restorer).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(restorer)
    }
}


pub fn restorer_apply(restorer: mmdeploy_restorer_t, img: &Mat, mat_count: i32) -> Result<ResResult, mmdeploy_status_t> {
    unsafe {
        let mat: *const mmdeploy_mat_t = &mmdeploy_mat_t {
            data: img.data() as *mut u8,
            height: img.rows(),
            width: img.cols(),
            channel: 3,
            format: mmdeploy_pixel_format_t_MMDEPLOY_PIXEL_FORMAT_BGR,
            type_: mmdeploy_data_type_t_MMDEPLOY_DATA_TYPE_UINT8,
        };

        let mut resResult = ResResult::new();

        let status: mmdeploy_status_t = mmdeploy_restorer_apply(restorer, mat, mat_count, resResult.results).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(resResult)
    }
}

pub fn restorer_release_result(res_result: ResResult, image_num: i32) {
    unsafe { mmdeploy_restorer_release_result(*res_result.results, image_num); }
}

pub fn restorer_release(restorer: mmdeploy_restorer_t) {
    unsafe { mmdeploy_restorer_destroy(restorer); }
}