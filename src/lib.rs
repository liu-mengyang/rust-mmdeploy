pub mod classifier;
pub mod detector;
pub mod segmentor;
pub mod pose_detector;
pub mod rotated_detector;
pub mod text_detector;
pub mod text_recognizer;


pub extern crate mmdeploy_sys as sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
