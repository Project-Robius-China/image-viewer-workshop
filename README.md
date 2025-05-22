# workshop 改进意见

1. 如果加载了错误格式的图片，则需要处理为一个「x」 图标展示

```
/Users/zhangalex/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/makepad-widgets-1.0.0/src/image_cache.rs:559:21 - load_image_file_by_path_and_data: Cannot load jpeg image from path: "/Users/zhangalex/Work/Projects/makepad/makepad-rustweek-workshop/image_viewer/resources/placeholder.jpg" JpgDecode(Error parsing image. Illegal start bytes:15393)
```

2. 加载的图片大小自适应

## `ImageRow` 和 `ImageGrid` 这两个 Makepad 组件区别

好的，我们来分析一下 `ImageRow` 和 `ImageGrid` 这两个 Makepad 组件。

它们共同构建了一个图像网格布局，其中：

- `ImageItem`: 是最基础的单元，用于显示单个图片。
- `ImageRow`: 负责将多个 `ImageItem` 水平排列成一行。
- `ImageGrid`: 负责将多个 `ImageRow` 垂直堆叠起来，形成一个网格。

让我们详细看看它们各自的定义和作用：

1.  **`ImageItem` (基础单元 - 在 `live_design!` 中定义，非独立组件)**

    - `live_design!` 定义：

      ```rust
      ImageItem = <View> {
          width: 256,
          height: 256,

          image = <Image> {
              width: Fill,
              height: Fill,
              fit: Biggest,
              source: (PLACEHOLDER) // 使用 "crate://self/resources/Rust.jpg"
          }
      }
      ```

    - **作用**: 这是一个简单的视图，包含一个 `Image` 控件，用于显示一张 256x256 大小的图片。它会作为 `ImageRow` 中每一项的模板。

2.  **`ImageRow` (图片行组件)**

    - `live_design!` 定义：

      ```rust
      ImageRow = {{ImageRow}} { // 引用下面的 Rust 结构体
          <PortalList> {        // 内部包含一个 PortalList
              height: 256,      // 行高，与 ImageItem 高度一致
              flow: Right,      // 关键：列表项从左到右排列

              ImageItem = <ImageItem> {} // PortalList 中每个子项的模板是 ImageItem
          }
      }
      ```

    - Rust 结构体与 `Widget` 实现：

      ```rust
      #[derive(Live, LiveHook, Widget)]
      pub struct ImageRow {
          #[deref]
          view: View, // 它本身是一个 View，其内容在 live_design 中定义
      }

      impl Widget for ImageRow {
          fn draw_walk(...) {
              // ...
              if let Some(mut list) = item.as_portal_list().borrow_mut() { // 获取 live_design 中定义的 PortalList
                  list.set_item_range(cx, 0, 4); // 关键：设置这一行有 4 个子项
                  while let Some(item_idx) = list.next_visible_item(cx) {
                      let item = list.item(cx, item_idx, live_id!(ImageItem)); // 获取或创建子项 (ImageItem)
                      item.draw_all(cx, &mut Scope::empty());                 // 绘制子项
                  }
              }
              // ...
          }
          // ...
      }
      ```

    - **作用与区别点**:
      - **结构**: `ImageRow` 本质上是一个 `View`，它在其 `live_design` 中包含了一个 `PortalList`。
      - **布局**: 这个 `PortalList` 的 `flow` 属性被设置为 `Right`，这意味着它内部的子项（即 `ImageItem`）会水平排列。
      - **内容**: `ImageRow` 的 `draw_walk` 方法通过 `list.set_item_range(cx, 0, 4)` 动态地指定它包含 **4 个** `ImageItem`。它使用 `PortalList` 来高效地渲染这些 `ImageItem`。
      - **小结**: `ImageRow` 负责创建和显示一行水平排列的图片。

3.  **`ImageGrid` (图片网格组件)**

    - `live_design!` 定义：

      ```rust
      ImageGrid = {{ImageGrid}} { // 引用下面的 Rust 结构体
          <PortalList> {          // 内部包含一个 PortalList
              flow: Down,         // 关键：列表项从上到下排列

              ImageRow = <ImageRow> {} // PortalList 中每个子项的模板是 ImageRow
          }
      }
      ```

    - Rust 结构体与 `Widget` 实现：

      ```rust
      #[derive(Live, LiveHook, Widget)]
      pub struct ImageGrid {
          #[deref]
          view: View, // 它本身是一个 View
      }

      impl Widget for ImageGrid {
          fn draw_walk(...) {
              // ...
              if let Some(mut list) = item.as_portal_list().borrow_mut() { // 获取 live_design 中定义的 PortalList
                  list.set_item_range(cx, 0, 3); // 关键：设置这个网格有 3 行
                  while let Some(row_idx) = list.next_visible_item(cx) {
                      let row = list.item(cx, row_idx, live_id!(ImageRow)); // 获取或创建子项 (ImageRow)
                      row.draw_all(cx, &mut Scope::empty());                // 绘制子项 (即 ImageRow)
                  }
              }
              // ...
          }
          // ...
      }
      ```

    - **作用与区别点**:
      - **结构**: `ImageGrid` 也是一个 `View`，它在其 `live_design` 中包含了一个 `PortalList`。
      - **布局**: 这个 `PortalList` 的 `flow` 属性被设置为 `Down`，这意味着它内部的子项（即 `ImageRow`）会垂直排列。
      - **内容**: `ImageGrid` 的 `draw_walk` 方法通过 `list.set_item_range(cx, 0, 3)` 动态地指定它包含 **3 个** `ImageRow`。
      - **小结**: `ImageGrid` 负责创建和显示一个由多行 `ImageRow` 组成的垂直列表，从而形成一个网格。

**总结区别**:

| 特性                            | `ImageRow`                            | `ImageGrid`                              |
| :------------------------------ | :------------------------------------ | :--------------------------------------- |
| **目的**                        | 显示一行水平排列的图片 (`ImageItem`s) | 显示一个垂直堆叠的图片行 (`ImageRow`s)   |
| **子项模板**                    | `ImageItem`                           | `ImageRow`                               |
| **内部 `PortalList` 的 `flow`** | `Right` (水平流动)                    | `Down` (垂直流动)                        |
| **子项数量 (代码中硬编码)**     | 4 个 `ImageItem`                      | 3 个 `ImageRow`                          |
| **结果**                        | 一行包含4张图片的横向列表             | 一个包含3行的垂直列表，每行是 `ImageRow` |

**最终效果**:

由于 `App` 使用了 `<ImageGrid> {}`，并且：

- `ImageGrid` 会创建 3 个 `ImageRow`。
- 每个 `ImageRow` 会创建 4 个 `ImageItem`。

所以，最终你会在界面上看到一个 **3 行 4 列** 的图片网格，总共显示 3 \* 4 = 12 个 `ImageItem`，每个 `ImageItem` 都显示 "Rust.jpg" 这张图片。

`PortalList` 的使用非常关键，它是一种虚拟列表，只渲染当前可见的列表项，这对于处理大量数据（比如成百上千张图片）时保持 UI 流畅性非常重要。在这个例子中，虽然只有 12 张图片，但展示了 `PortalList` 的基本用法。

## 图片异步加载机制

```
fn load_image_file_by_path_async_impl
```

`load_image_file_by_path_async_impl` 函数是 Makepad 中用于**异步加载图片文件**的核心实现。它的目标是在不阻塞主 UI 线程的情况下，从文件系统加载和解码图片。

下面是这个函数实现机制的详细解读：

**1. 检查缓存 (Cache Check)**

```rust
if let Some(texture_entry) = cx.get_global::<ImageCache>().map.get(image_path) {
    match texture_entry {
        ImageCacheEntry::Loaded(texture) => {
            // 图片已加载并缓存
            let texture = texture.clone();
            self.set_texture(Some(texture), id); // 直接使用缓存的 Texture
            return Ok(AsyncLoadResult::Loaded);  // 返回已加载状态
        }
        ImageCacheEntry::Loading(w, h) => {
            // 图片正在加载中
            return Ok(AsyncLoadResult::Loading(*w, *h)); // 返回加载中状态及预估尺寸
        }
    }
}
```

- **目的**: 提高性能，避免重复加载。
- **行为**:
  - 首先，函数会检查全局的 `ImageCache` 中是否已经存在该 `image_path` 的条目。
  - **如果已加载 (`ImageCacheEntry::Loaded`)**:
    - 直接从缓存中获取 `Texture` 对象。
    - 调用 `self.set_texture(...)` 将这个 `Texture` 设置到请求该图片的组件上（通过 `id` 区分）。
    - 返回 `Ok(AsyncLoadResult::Loaded)`，表示图片已成功加载并应用。
  - **如果正在加载中 (`ImageCacheEntry::Loading`)**:
    - 这意味着其他地方已经触发了对同一张图片的加载请求，但尚未完成。
    - 返回 `Ok(AsyncLoadResult::Loading(w, h))`，告知调用者图片正在加载，并提供预期的宽度 `w` 和高度 `h`（这些尺寸是在首次请求加载时通过读取文件头获得的）。这允许 UI 在图片加载完成前预留空间或显示占位符。

**2. 初始化异步加载 (If Not Cached)**

如果图片既没有被加载，也不在加载中，则开始新的异步加载流程：

```rust
else {
    // 确保线程池存在
    if cx.get_global::<ImageCache>().thread_pool.is_none() {
        cx.get_global::<ImageCache>().thread_pool = Some(TagThreadPool::new(cx, cx.cpu_cores().max(3) - 2));
    }

    // 1. 同步获取图片尺寸 (用于占位)
    let (w, h) = Self::image_size_by_path(image_path)?; // 可能返回 ImageError::PathNotFound 等

    // 2. 将图片标记为正在加载状态，并存入缓存
    cx.get_global::<ImageCache>().map.insert(image_path.into(), ImageCacheEntry::Loading(w, h));

    // 3. 将加载任务提交到线程池
    cx.get_global::<ImageCache>().thread_pool.as_mut().unwrap().execute_rev(image_path.into(), move |image_path_clone| {
        // ... 线程池中的工作代码 ...
    });

    // 4. 返回加载中状态
    Ok(AsyncLoadResult::Loading(w, h))
}
```

- **线程池初始化**:

  - `if cx.get_global::<ImageCache>().thread_pool.is_none()`: 检查全局 `ImageCache` 是否已经有一个 `TagThreadPool`。
  - 如果没有，就创建一个新的线程池。`TagThreadPool` 是 Makepad 用于管理后台任务的工具，`cx.cpu_cores().max(3) - 2` 是一种启发式的线程数设置方式，尝试在不占用过多 CPU 资源的情况下进行并行处理。

- **获取图片尺寸 (同步)**:

  - `let (w, h) = Self::image_size_by_path(image_path)?;`
  - **关键点**: 在将实际的解码任务放到后台线程之前，它会**同步地**尝试读取图片文件的头部信息来获取图片的宽度和高度。
  - 这是通过 `Self::image_size_by_path` 实现的，该函数会打开文件，只读取足够的数据来解析图片头（例如 JPEG 或 PNG 的元数据），而不会解码整个图片。
  - **原因**: 这样可以快速得到图片的尺寸，UI 可以立即使用这些尺寸来布局或显示一个正确大小的占位符，即使用户界面看起来不会因为图片加载而跳动。
  - 如果此步骤失败（例如文件不存在），会直接返回错误 `Err(ImageError::PathNotFound(...))`。

- **更新缓存状态**:

  - `cx.get_global::<ImageCache>().map.insert(image_path.into(), ImageCacheEntry::Loading(w, h));`
  - 将该图片的条目添加到 `ImageCache` 中，状态标记为 `ImageCacheEntry::Loading(w, h)`，并存储刚刚获取到的宽高。这样，如果在此图片完成加载前，又有其他组件请求同一张图片，它们会直接进入上面第一步的 `ImageCacheEntry::Loading` 分支。

- **提交任务到线程池**:

  - `cx.get_global::<ImageCache>().thread_pool.as_mut().unwrap().execute_rev(image_path.into(), move |image_path_clone| { ... });`
  - 这是异步操作的核心。
  - `execute_rev` 方法将一个闭包（包含实际加载和解码逻辑）提交到线程池中执行。
  - `image_path.into()` 作为任务的标签或标识符。
  - `move |image_path_clone| { ... }`: 这个闭包会在后台线程中执行。`image_path_clone` 是 `image_path` 的一个克隆，因为闭包需要拥有其捕获的变量。

- **线程池中的工作 (闭包内部逻辑)**:

  ```rust
  if let Ok(mut f) = File::open(&image_path_clone) { // 尝试打开文件
      let mut data = Vec::new();
      match f.read_to_end(&mut data) { // 读取整个文件内容到内存
          Ok(_len) => {
              // 根据文件扩展名选择解码器
              if image_path_clone.extension().map(|s| s == "jpg").unwrap_or(false) {
                  match ImageBuffer::from_jpg(&*data) { // 解码 JPEG
                      Ok(decoded_data) => {
                          Cx::post_action(AsyncImageLoad { // 发送成功结果
                              image_path: image_path_clone,
                              result: RefCell::new(Some(Ok(decoded_data)))
                          });
                      }
                      Err(err) => { /* 发送解码错误结果 */ }
                  }
              } else if image_path_clone.extension().map(|s| s == "png").unwrap_or(false) {
                  match ImageBuffer::from_png(&*data) { // 解码 PNG
                      Ok(decoded_data) => { /* 发送成功结果 */ }
                      Err(err) => { /* 发送解码错误结果 */ }
                  }
              } else {
                  // 不支持的格式
                  /* 发送不支持格式的错误结果 */
              }
          }
          Err(_err) => {
              // 文件读取错误
              /* 发送文件读取错误结果 */
          }
      }
  } else {
      // 文件打开错误 (路径未找到)
      /* 发送路径未找到错误结果 */
  }
  ```

  - **文件操作**: 在后台线程中，尝试打开并读取整个图片文件的内容到 `data` (一个 `Vec<u8>`)。
  - **解码**:
    - 根据文件的扩展名 (`.jpg` 或 `.png`)，调用相应的解码函数 (`ImageBuffer::from_jpg` 或 `ImageBuffer::from_png`)。这些解码函数会把原始文件字节流转换为 `ImageBuffer` 结构（包含像素数据、宽度、高度）。
  - **发送结果回主线程**:
    - **关键点**: 无论加载和解码成功还是失败，都会通过 `Cx::post_action(...)` 将结果发送回 Makepad 的主事件循环。
    - `AsyncImageLoad` 是一个包含 `image_path` 和 `result` (一个 `RefCell<Option<Result<ImageBuffer, ImageError>>>`) 的结构体。
    - `Cx::post_action` 是 Makepad 提供的机制，用于从其他线程安全地向主 UI 线程发送消息（Action）。主线程在处理事件循环时会接收这些 Action。
    - 这个结果（成功时是 `Ok(ImageBuffer)`，失败时是 `Err(ImageError)`) 会在主线程的 `handle_event` 中被捕获（通常是在实现了 `ImageCacheImpl` 的某个 widget 或 `App` 结构中），然后由 `process_async_image_load` 函数处理。`process_async_image_load` 会将解码后的 `ImageBuffer` 转换成 `Texture`，并更新 `ImageCache` 中的条目为 `ImageCacheEntry::Loaded(texture)`，同时触发 UI 重绘。

- **立即返回 `Loading` 状态**:
  - `Ok(AsyncLoadResult::Loading(w, h))`
  - 在将任务提交到线程池后，`load_image_file_by_path_async_impl` 函数**立即返回** `AsyncLoadResult::Loading`。它不会等待后台线程完成。
  - 这意味着调用者（通常是 `Image` widget）知道图片正在加载，并可以使用返回的 `w` 和 `h` 来更新其布局。

**总结流程**:

1.  **请求图片**: UI 组件 (如 `Image`) 需要显示一张图片，调用此异步加载函数。
2.  **缓存检查**:
    - **命中已加载**: 直接使用缓存的 `Texture`，返回 `Loaded`。
    - **命中正在加载**: 返回 `Loading` 及尺寸，等待加载完成。
3.  **开始新加载 (缓存未命中)**:
    - 确保线程池已创建。
    - **同步**读取文件头获取图片尺寸。
    - 在缓存中将图片标记为 `Loading` 状态，并存入尺寸。
    - 将实际的文件读取和解码任务封装成一个闭包，提交到后台线程池。
    - **立即**返回 `Loading` 状态及尺寸给调用者。
4.  **后台处理 (线程池)**:
    - 后台线程执行闭包：打开文件、读取内容、解码图片。
    - 将解码结果 (成功或失败) 通过 `Cx::post_action` 发送回主线程。
5.  **主线程处理结果**:
    - 主线程的事件循环接收到 `AsyncImageLoad` Action。
    - `process_async_image_load` 被调用。
    - 如果成功，将 `ImageBuffer` 转换为 `Texture`，更新缓存为 `Loaded(texture)`，并触发 UI 重绘以显示新加载的图片。
    - 如果失败，记录错误。

这种机制确保了即使加载大图片或网络图片（虽然这里是本地文件），UI 线程也不会被阻塞，应用保持响应。同时，通过预先获取尺寸和缓存机制，提升了用户体验和效率。
