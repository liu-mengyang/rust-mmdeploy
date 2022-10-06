cargo run --example classifier cpu ../mmdeploy-converted-models/resnet ./images/demos/mmcls_demo.jpg
cargo run --example detector cpu ../mmdeploy-converted-models/faster-rcnn-ort ./images/demos/mmdet_demo.jpg
cargo run --example segmentor cpu ../mmdeploy-converted-models/deeplabv3 ./images/demos/mmseg_demo.png
cargo run --example pose_detector cpu ../mmdeploy-converted-models/hrnet ./images/demos/mmpose_demo.jpg
cargo run --example rotated_detector cpu ../mmdeploy-converted-models/retinanet ./images/demos/mmrotate_demo.jpg
cargo run --example ocr cpu ../mmdeploy-converted-models/dbnet ../mmdeploy-converted-models/crnn ./images/demos/mmocr_demo.jpg
cargo run --example restorer cpu ../mmdeploy-converted-models/edsr ./images/demos/mmediting_demo.png