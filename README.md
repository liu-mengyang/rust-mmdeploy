![](images/rust-mmdeploy-logo.png)

[![crates.io](https://img.shields.io/crates/v/mmdeploy)](https://crates.io/crates/mmdeploy)

## Introduction

Safe MMDeploy Rust wrapper. This project aims to provide a Rust wrapper for MMDeploy>=1.0.0.

## News

- (2024.12.24) Bump to MMDeploy v1.1.0.
- (2022.9.29) This repo has been added to the [OpenMMLab ecosystem](https://openmmlab.com/ecosystem).
- (2022.9.27) This repo has been added to the [MMDeploy CI](https://github.com/open-mmlab/mmdeploy/blob/master/.github/workflows/rust_api.yml).

## Prerequisites

To make sure the building of this repo successful, you should install some pre-packages.

The following guidance is tested on Ubuntu OS on x86 device.

**Step 0.** Install Rust if you don't have.

```bash
apt install curl
curl https://sh.rustup.rs -sSf | sh
```

**Step 1.** Install Clang and Rust required by `Bindgen`.

```bash
apt install llvm-dev libclang-dev clang
```

**Step 2.** Download and install pre-built mmdeploy package. Currently, `mmdeploy-sys` is built upon the pre-built package of `mmdeploy` so this repo only supports OnnxRuntime and TensorRT backends. Don't be disappoint, the script of building from source is ongoing, and after finishing that we can deploy models with all backends supported by `mmdeploy` in Rust.

```bash
apt install wget
```

**If you want to deploy models with OnnxRuntime:**

```bash
# Download and link to MMDeploy-onnxruntime pre-built package
wget https://github.com/open-mmlab/mmdeploy/releases/download/v1.1.0/mmdeploy-1.1.0-linux-x86_64-cuda11.3.tar.gz
tar -zxvf mmdeploy-1.1.0-linux-x86_64-cuda11.3.tar.gz
cd mmdeploy-1.1.0-linux-x86_64-cuda11.3
export MMDEPLOY_DIR=$(pwd)
export ONNXRUNTIME_DIR=$(pwd)/thirdparty/onnxruntime
export LD_LIBRARY_PATH=$ONNXRUNTIME_DIR/lib:$LD_LIBRARY_PATH
# Download and link to TensorRT engine
# !!! Download TensorRT-8.2.3.0 CUDA 11.x tar package from NVIDIA, and extract it to the current directory. This link maybe helpful: https://developer.nvidia.com/nvidia-tensorrt-8x-download.
export TENSORRT_DIR=$(pwd)/thirdparty/tensorrt
export LD_LIBRARY_PATH=$TENSORRT_DIR/lib:$LD_LIBRARY_PATH
export CUDNN_DIR=/usr/local/cuda
export LD_LIBRARY_PATH=$CUDNN_DIR/lib64:$LD_LIBRARY_PATH
```

If you build MMDeploy SDK from source, then you should set MMDEPLOY_DIR and LD_LIBRARY_PATH as follows:

```bash
export MMDEPLOY_DIR=/the/path/to/mmdeploy/build/install
export LD_LIBRARY_PATH=$MMDEPLOY_DIR/lib:$LD_LIBRARY_PATH
```
then you need to configure the path of TensorRT, ONNXRUNTIME, CUDA and cuDNN as follows:

```bash
export TENSORRT_DIR=/the/path/to/tensorrt
export LD_LIBRARY_PATH=${TENSORRT_DIR}/lib:$LD_LIBRARY_PATH
export ONNXRUNTIME_DIR=/the/path/to/onnxruntime
export LD_LIBRARY_PATH=${ONNXRUNTIME_DIR}/lib:$LD_LIBRARY_PATH
export CUDNN_DIR=/usr/local/cuda
export LD_LIBRARY_PATH=$CUDNN_DIR/lib64:$LD_LIBRARY_PATH
```

**Step 3.** (Optional) Install OpenCV required by examples.

```bash
apt install libopencv-dev
```

**Step 4.** (Optional) Download converted onnx models by `mmdeploy-converted-models`.
```bash
apt install git-lfs
git clone https://github.com/liu-mengyang/mmdeploy-converted-models --depth=1
```


## Quickstart

Please read the previous section to make sure the required packages have been installed before using this crate.

Update your *Cargo.toml*

```toml
mmdeploy = "1.1.0"
```

## APIs for MM Codebases

Good news: Now, you can use Rust language to build your fantastic applications powered by MMDeploy!
Take a look by running some examples! In these examples, `CPU` is the default inference device. If you choose to deploy models on `GPU`, you will replace all `cpu` in test commands with `cuda`.

### Convert Models

You can

* Directly use converted models [here](https://github.com/liu-mengyang/mmdeploy-converted-models) ^_^
* Or follow [MMDeploy documentation](https://mmdeploy.readthedocs.io/en/latest/get_started.html#convert-model) to install and convert appropriate models

### Classifier API

Deploy image classification models converted by MMDeploy.

The example deploys a ResNet model converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example classifier cpu ../mmdeploy-converted-models/resnet ./images/demos/mmcls_demo.jpg
```

### Detector API

Deploy object detection models converted by MMDeploy.

The example deploys a FasterRCNN model converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example detector cpu ../mmdeploy-converted-models/faster-rcnn-ort ./images/demos/mmdet_demo.jpg
```

A rendered result we can take a look located in the current directory and is named `output_detection.png`.

![](images/output_detection.png)

### Segmentor API

Deploy object segmentation models converted by MMDeploy.

The example deploys a DeepLabv3 model converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example segmentor cpu ../mmdeploy-converted-models/deeplabv3 ./images/demos/mmseg_demo.png
```

A rendered result we can take a look located in the current directory and is named `output_segmentation.png`.

![](images/output_segmentation.png)

### Pose detector API

Deploy pose detection models converted by MMDeploy.

The example deploys an HRNet model converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example pose_detector cpu ../mmdeploy-converted-models/hrnet ./images/demos/mmpose_demo.jpg
```

A rendered result we can take a look located in the current directory and is named `output_pose.png`.

![](images/output_pose.png)

### Rotated detector API

Deploy rotated detection models converted by MMDeploy.

The example deploys a RetinaNet model converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example rotated_detector cpu ../mmdeploy-converted-models/retinanet ./images/demos/mmrotate_demo.jpg
```

A rendered result we can take a look located in the current directory and is named `output_rotated_detection.png`.

![](images/output_rotated_detection.png)

### OCR API

Deploy text detection and text recognition models converted by MMDeploy.

The example deploys a DBNet model for detection and a CRNN model for recognition both converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example ocr cpu ../mmdeploy-converted-models/dbnet ../mmdeploy-converted-models/crnn ./images/demos/mmocr_demo.jpg
```

A rendered result we can take a look located in the current directory and is named `output_ocr.png`.

![](images/output_ocr.png)

### Restorer API

Deploy restorer models converted by MMDeploy.

The example deploys an EDSR model for restoration converted by the ONNXRUNTIME target on a CPU device.

```bash
cargo run --example restorer cpu ../mmdeploy-converted-models/edsr ./images/demos/mmediting_demo.png
```

A rendered result we can take a look located in the current directory and is named `output_restorer.png`.

![](images/output_restorer.png)

### TOSupport List

- [x] Classifier
- [x] Detector
- [x] Segmentor
- [x] Pose Detector
- [x] Rotated Detector
- [x] Text Detector
- [x] Text Recognizer
- [x] Restorer

### TODO List

- [x] PR for contributing a rust-mmdeploy-CI into MMDeploy
- [x] Test with TensorRT prebuilt package
- [ ] Bump to the latest MMDeploy