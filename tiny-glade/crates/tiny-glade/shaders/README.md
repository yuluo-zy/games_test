# 应用呈现 shaders

`compiled/` 是当前正式呈现路径使用的 shader 包，由本工程 `shaders/blit/resolve*_to_swapchain.raster.hlsl` 构建。ShaderHost 在加载时直接使用随应用打包的版本，不回退到安装树中只支持相同尺寸的旧 resolve。仍共用原来的反射、bindless、管线 Pending、Collect 和图提交路径。

窗口 resize 可发生在主图提交与交换链获取之间。当前 shader 将目标像素中心映射到源图，再进行原有 MSAA 平均和线性到 sRGB 编码；同尺寸仍逐像素读取。缩放期间使用最近邻，下一张正确尺寸主图恢复原生像素。翻转在目标坐标上完成。该尺寸适配属于当前工程修复，不宣称为原版等尺寸行为的逆向成果。

重建四个包（两个采样类型 × 两个深度变体）：

```sh
cargo run -p shader-compiler --example build_presentation -- /path/to/dxc /exclusive/work/dir
```

构建输入、编译器版本和产物 SHA 见 `compiled/manifest.json`。像素回归位于 `tests/original_present_gpu.rs`，覆盖同尺寸、放大、缩小、1/4 采样、翻转和所有输出像素。测试读取 GPU 图像，不依赖屏幕录制权限。
