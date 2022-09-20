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

pub struct Pose {
    pub point: *mut mmdeploy_point_t,
    pub score: *mut f32,
    pub length: i32,
}

impl Pose {
    pub fn from(pose_detection: mmdeploy_pose_detection_t) -> Pose {
        Pose {
            point: pose_detection.point,
            score: pose_detection.score,
            length: pose_detection.length,
        }
    }
    
    pub fn get_point(&self, pose_index: isize) -> Point {
        unsafe {Point::from(*self.point.offset(pose_index))}
    }
}

pub struct PoseResult {
    pub results: *mut *mut mmdeploy_pose_detection_t,
}

impl PoseResult {
    pub fn new() -> PoseResult {
        PoseResult {
            results: Box::into_raw(Box::new(mmdeploy_pose_detection_t::new())) as *mut *mut mmdeploy_pose_detection_t,
        }
    }

    pub fn get_result(&self, image_index:isize, box_index: isize) -> Pose {
        unsafe{
            Pose::from(*(*self.results.offset(image_index)).offset(box_index))
        }
    }

    
}

pub fn pose_detector_create_by_path(model_path: &str, device_name: &str, device_id: i32) -> Result<mmdeploy_pose_detector_t, mmdeploy_status_t> {
    unsafe{
        let mut pose_detector: mmdeploy_pose_detector_t = &mut mmdeploy_pose_detector::new();

        let model_path_cstr = CString::new(&model_path[..]).unwrap();
        let model_path_i8: *const c_char = model_path_cstr.as_ptr() as *const c_char;

        let device_name_cstr = CString::new(&device_name[..]).unwrap();
        let device_name_i8: *const c_char = device_name_cstr.as_ptr() as *const c_char;

        let status: mmdeploy_status_t = mmdeploy_pose_detector_create_by_path(model_path_i8, device_name_i8, device_id, &mut pose_detector).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(pose_detector)
    }
}


pub fn pose_detector_apply(pose_detector: mmdeploy_pose_detector_t, img: &Mat, mat_count: i32) -> Result<PoseResult, mmdeploy_status_t> {
    unsafe {
        let mat: *const mmdeploy_mat_t = &mmdeploy_mat_t {
            data: img.data() as *mut u8,
            height: img.rows(),
            width: img.cols(),
            channel: 3,
            format: mmdeploy_pixel_format_t_MMDEPLOY_PIXEL_FORMAT_BGR,
            type_: mmdeploy_data_type_t_MMDEPLOY_DATA_TYPE_UINT8,
        };

        let mut poseResult = PoseResult::new();

        let status: mmdeploy_status_t = mmdeploy_pose_detector_apply(pose_detector, mat, mat_count, poseResult.results).try_into().unwrap();

        if status != mmdeploy_status_t_MMDEPLOY_SUCCESS {
            return Err(status);
        }
        Ok(poseResult)
    }
}

pub fn pose_detector_release_result(pose_result: PoseResult, image_num: i32) {
    unsafe { mmdeploy_pose_detector_release_result(*pose_result.results, image_num); }
}

pub fn pose_detector_release(pose_detector: mmdeploy_pose_detector_t) {
    unsafe { mmdeploy_pose_detector_destroy(pose_detector); }
}