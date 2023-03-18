# model file discription

these model file are copy from opencv tutorials: https://github.com/opencv/opencv/blob/4.x/doc/tutorials/dnn/dnn_text_spotting/dnn_text_spotting.markdown

below are copy from there docs:

## TextRecognitionModel:
- crnn.onnx:
url: https://drive.google.com/uc?export=dowload&id=1ooaLR-rkTl8jdpGy1DoQs0-X0lQsB6Fj
sha: 270d92c9ccb670ada2459a25977e8deeaf8380d3,
alphabet_36.txt: https://drive.google.com/uc?export=dowload&id=1oPOYx5rQRp8L6XQciUwmwhMCfX0KyO4b
parameter setting: -rgb=0;
description: The classification number of this model is 36 (0~9 + a~z).

- crnn_cs.onnx:
url: https://drive.google.com/uc?export=dowload&id=12diBsVJrS9ZEl6BNUiRp9s0xPALBS7kt
sha: a641e9c57a5147546f7a2dbea4fd322b47197cd5
alphabet_94.txt: https://drive.google.com/uc?export=dowload&id=1oKXxXKusquimp7XY1mFvj9nwLzldVgBR
parameter setting: -rgb=1;
description: The classification number of this model is 94 (0~9 + a~z + A~Z + punctuations).

- crnn_cs_CN.onnx:
url: https://drive.google.com/uc?export=dowload&id=1is4eYEUKH7HR7Gl37Sw4WPXx6Ir8oQEG
sha: 3940942b85761c7f240494cf662dcbf05dc00d14
alphabet_3944.txt: https://drive.google.com/uc?export=dowload&id=18IZUUdNzJ44heWTndDO6NNfIpJMmN-ul
parameter setting: -rgb=1;
description: The classification number of this model is 3944 (0~9 + a~z + A~Z + Chinese characters + special characters).
             The training dataset is ReCTS (https://rrc.cvc.uab.es/?ch=12).
More models can be found in here, which are taken from clovaai. You can train more models by CRNN, and convert models by torch.onnx.export.

## TextDetectionModel:
- DB_IC15_resnet50.onnx:
url: https://drive.google.com/uc?export=dowload&id=17_ABp79PlFt9yPCxSaarVc_DKTmrSGGf
sha: bef233c28947ef6ec8c663d20a2b326302421fa3
recommended parameter setting: -inputHeight=736, -inputWidth=1280;
description: This model is trained on ICDAR2015, so it can only detect English text instances.

- DB_IC15_resnet18.onnx:
url: https://drive.google.com/uc?export=dowload&id=1vY_KsDZZZb_svd5RT6pjyI8BS1nPbBSX
sha: 19543ce09b2efd35f49705c235cc46d0e22df30b
recommended parameter setting: -inputHeight=736, -inputWidth=1280;
description: This model is trained on ICDAR2015, so it can only detect English text instances.

- DB_TD500_resnet50.onnx:
url: https://drive.google.com/uc?export=dowload&id=19YWhArrNccaoSza0CfkXlA8im4-lAGsR
sha: 1b4dd21a6baa5e3523156776970895bd3db6960a
recommended parameter setting: -inputHeight=736, -inputWidth=736;
description: This model is trained on MSRA-TD500, so it can detect both English and Chinese text instances.

- DB_TD500_resnet18.onnx:
url: https://drive.google.com/uc?export=dowload&id=1sZszH3pEt8hliyBlTmB-iulxHP1dCQWV
sha: 8a3700bdc13e00336a815fc7afff5dcc1ce08546
recommended parameter setting: -inputHeight=736, -inputWidth=736;
description: This model is trained on MSRA-TD500, so it can detect both English and Chinese text instances.

- EAST:
Download link: https://www.dropbox.com/s/r2ingd0l3zt8hxs/frozen_east_text_detection.tar.gz?dl=1
This model is based on https://github.com/argman/EAST
