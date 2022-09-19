pub mod classifier;
pub mod detector;
pub mod segmentor;


pub extern crate mmdeploy_sys as sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
