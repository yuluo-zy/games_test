# utils/load_json.rs

> 文件作用：**JSON→Mesh 网格加载器（游戏自有，共享 utils 无此实现；68 调用方依赖本实现）**。
> `load_json_as_mesh`（4 泛型实例：`&str`/`PathBuf`/`String`/`&PathBuf`，入参一律为**路径**）
> 读文件 → 自定义 JSON 解析（FUN_141405930，节点布局 serde_json-Value 形状但容器为 cap-first
> Vec）→ 组装 bevy 风格 Mesh。`read_attribute_from_json`（@0x14083e210）解析单个属性的
> `{name, type: ["int"|"float", N], buffer}` 描述符。全算法流程图与逐字常量见
> `evidence/country-core-deep/utils/ALGORITHMS.md` §1。

函数总计 19：业务 5 / 内联无独立符号 0 / 派生胶水 14

## 算法总览（4 实例同体，仅输入转换不同）

```
load_json_as_mesh<T: AsRef<Path>>(path) -> Mesh (0x78 B, label="load_json_as_mesh")
  1. 输入转 &str（String/PathBuf 实例解包；&PathBuf 实例经 FUN_140d76ed0 = Path::to_str）
  2. FUN_140d59140 = fs::read_to_string(path)            // 失败 → panic "Couldn't find  {path}"
  3. FUN_141405930 = 自定义 JSON 文本解析 → Value 树      // 失败(tag 6=Err) → panic "Couldn't parse  {path}"
  4. Mesh::new（FUN_140d61b00，RandomState 密钥取线程局部 + label "load_json_as_mesh"）
  5. indices = root["indices"]["buffer"]（两次嵌套查键 "indices"(7) + "buffer"(6)，字面量共享
     "indicesbuffer" @0x142ae800f）→ 必须 Array(tag 4) 否则 panic(load_json.rs)
     逐元素校验 {tag@0==2(Number), 子tag@8==0(PosInt)} → 提取 u32 → Vec<u32>
     SIMD 8-lane argmax 求 max_index；空数组 → max_index = 0xffffffff
  6. attrs = root["attributes"]（10 字符键 @0x142ae8038）→ 必须 Array
     逐元素校验 {tag@0==3(String)} → 拷 {ptr,len} 名字对
     对每个属性: read_attribute_from_json(attrs[i], name) → {tag, Vec} 24B
        tag==5（不识别）→ Err → 尾段 .unwrap() panic
        断言 value_count > max_index（`values.array_length() as i32 > max_index`
        @0x142ae8078，load_json.rs:58）→ 否则 panic
     FUN_140a35240 = Mesh::set_attribute：hashbrown 查名（SipHash FUN_1405b1a70 + SIMD 16B 组比较
        + memcmp FUN_142923bf0）→ 已存在：按判别元素字节宽 4/8/0xc/0x10 extend_from_slice；
        不存在：insert 新条目（反编译证据 `utils/decomp/decomp-0x140a35240.txt`）
  7. 释放根 Value（tag 2 无动作 / 3 释放串 / 4 释放 Vec(FUN_140888d70) / else 释放对象
     FUN_140898f40）→ 返回 Mesh {indices cap-first Vec<u32>@0, source_file=None(0x8000…@0x18),
     attributes HashMap@0x38, label="load_json_as_mesh"@0x68}
```

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `load_json_as_mesh<String>` | 0x140b10ed0 | 5 | 路径→Mesh（消费释放 String） | **O** | 解包 String{cap,ptr,len} + 上述流程 |
| `load_json_as_mesh<&PathBuf>` | 0x140b11f50 | 5 | 借用 PathBuf→Mesh | **O** | Path::to_str + 流程（借用不释放） |
| `load_json_as_mesh<&str>` | 0x140b0eeb0 | 5 | 路径&str→Mesh（68 调用方） | **O** | 直读 {ptr,len}；panic `Couldn't find  {p}` / `Couldn't parse  {p}` |
| `load_json_as_mesh<PathBuf>` | 0x140b0fe60 | 5 | PathBuf→Mesh | **O** | 同（释放 PathBuf） |
| `read_attribute_from_json` | 0x14083e210 | 51 | 按名读属性描述符→{tag,Vec<f32>} | **O** | 见 ALGORITHMS.md §1.2 |

## 派生胶水（14 项：closure 等）

- 4 实例共用同体（ICF 合并）；`FUN_1414744d0` = 按字符串键查 JSON 对象（miss → 全局默认值
  `*0x143630758`）；`FUN_140d79f90` = fmt::format（Arguments 表渲染，兼用于 panic 文案）。
- 入口 `func_0x000140c16a00` = 读全局字节 `*0x14362e880`（puffin profile scope 门控，
  U：写入者未找到）选择两条输入路径：分支 B 直读文件；分支 A 走 scope/backtrace 构造后并入
  同一解析流（puVar27=1 尾部多一次 Arc 释放）。
