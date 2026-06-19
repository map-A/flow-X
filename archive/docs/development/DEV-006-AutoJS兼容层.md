# DEV-006: AutoJS兼容层

**任务ID**：DEV-006  
**负责角色**：Python/JS开发  
**优先级**：P1（高）  
**预计工时**：30小时  
**依赖**：DEV-005  
**阶段**：Phase 2 (Week 8)

---

## 任务目标

实现AutoJS核心API的80%兼容，让现有AutoJS脚本能够在FlowX上运行。

---

## AutoJS API优先级

### P0: 必须实现（MVP）

| API | 说明 | 实现难度 |
|-----|------|---------|
| `click(x, y)` | 点击坐标 | 低 |
| `swipe(x1,y1,x2,y2,duration)` | 滑动 | 低 |
| `text(str).findOne()` | 文本查找 | 中 |
| `id(str).findOne()` | ID查找 | 中 |
| `sleep(ms)` | 延迟 | 低 |
| `app.launch(pkg)` | 启动应用 | 低 |
| `toast(msg)` | 提示消息 | 低 |
| `auto.waitFor()` | 等待服务 | 低 |

### P1: 重要（Phase 2）

| API | 说明 | 实现难度 |
|-----|------|---------|
| `longClick(x, y)` | 长按 | 低 |
| `input(text)` | 输入文本 | 低 |
| `className(name).findOne()` | 类名查找 | 中 |
| `desc(str).findOne()` | 描述查找 | 中 |
| `captureScreen()` | 截图 | 中 |
| `findColor(img, color)` | 找色 | 高 |

### P2: 可选（后续）

| API | 说明 | 实现难度 |
|-----|------|---------|
| `http.get(url)` | HTTP请求 | 中 |
| `files.read(path)` | 文件操作 | 低 |
| `threads.start()` | 多线程 | 高 |
| `ui.layout()` | UI布局 | 高 |

---

## 完整API实现

### 1. 全局函数 (`stdlib/globals.js`)

```javascript
// 基础操作
function click(x, y) {
    return __native_click(x, y);
}

function longClick(x, y) {
    return __native_long_click(x, y, 1000);
}

function swipe(x1, y1, x2, y2, duration) {
    duration = duration || 300;
    return __native_swipe(x1, y1, x2, y2, duration);
}

function press(x, y, duration) {
    return longClick(x, y);
}

function sleep(ms) {
    __native_sleep(ms);
}

function toast(message) {
    __native_toast(message);
}

// 等待函数
function waitForActivity(activity, timeout) {
    timeout = timeout || 10000;
    return __native_wait_for_activity(activity, timeout);
}

function waitForPackage(pkg, timeout) {
    timeout = timeout || 10000;
    return __native_wait_for_package(pkg, timeout);
}
```

### 2. 选择器系统 (`stdlib/selector.js`)

```javascript
// 文本选择器
function text(str) {
    return new UiSelector("text", str);
}

function textContains(str) {
    return new UiSelector("textContains", str);
}

function textStartsWith(str) {
    return new UiSelector("textStartsWith", str);
}

function textEndsWith(str) {
    return new UiSelector("textEndsWith", str);
}

function textMatches(regex) {
    return new UiSelector("textMatches", regex);
}

// ID选择器
function id(resId) {
    return new UiSelector("id", resId);
}

function idContains(str) {
    return new UiSelector("idContains", str);
}

// 类名选择器
function className(name) {
    return new UiSelector("className", name);
}

// 描述选择器
function desc(str) {
    return new UiSelector("desc", str);
}

function descContains(str) {
    return new UiSelector("descContains", str);
}

// 选择器类
class UiSelector {
    constructor(type, value) {
        this.type = type;
        this.value = value;
        this.filters = [];
    }
    
    // 查找单个
    findOne(timeout) {
        timeout = timeout || 0;
        let result = __native_find_element(this.type, this.value, timeout);
        if (result) {
            return new UiObject(result);
        }
        return null;
    }
    
    // 查找所有
    find() {
        let results = __native_find_elements(this.type, this.value);
        return results.map(r => new UiObject(r));
    }
    
    // 等待出现
    waitFor() {
        return this.findOne(10000);
    }
    
    // 链式调用
    clickable(b) {
        this.filters.push({ type: "clickable", value: b !== false });
        return this;
    }
    
    scrollable(b) {
        this.filters.push({ type: "scrollable", value: b !== false });
        return this;
    }
}

// UI对象
class UiObject {
    constructor(data) {
        this.bounds = data.bounds;
        this.text = data.text;
        this.id = data.id;
        this.desc = data.desc;
        this.className = data.className;
        this.clickable = data.clickable;
        this.scrollable = data.scrollable;
    }
    
    click() {
        let center = this.center();
        return click(center.x, center.y);
    }
    
    longClick() {
        let center = this.center();
        return longClick(center.x, center.y);
    }
    
    setText(text) {
        this.click();
        sleep(100);
        return __native_input_text(text);
    }
    
    center() {
        return {
            x: this.bounds.left + (this.bounds.right - this.bounds.left) / 2,
            y: this.bounds.top + (this.bounds.bottom - this.bounds.top) / 2
        };
    }
    
    parent() {
        // TODO: 实现父节点查找
        return null;
    }
    
    child(index) {
        // TODO: 实现子节点查找
        return null;
    }
}
```

### 3. 应用管理 (`stdlib/app.js`)

```javascript
const app = {
    // 启动应用
    launch: function(packageName) {
        return __native_launch_app(packageName);
    },
    
    launchApp: function(appName) {
        return this.launch(appName);
    },
    
    // 获取当前应用
    getPackageName: function() {
        return __native_get_current_package();
    },
    
    // 获取应用名称
    getAppName: function(packageName) {
        return __native_get_app_name(packageName);
    },
    
    // 打开设置
    openAppSetting: function(packageName) {
        return __native_open_app_setting(packageName);
    }
};
```

### 4. 自动化服务 (`stdlib/auto.js`)

```javascript
const auto = {
    // 等待服务
    waitFor: function() {
        // FlowX默认已启动
        return true;
    },
    
    // 设置服务
    setMode: function(mode) {
        // normal, fast, secure
        return true;
    },
    
    // 设置标志
    setFlags: function(flags) {
        return true;
    }
};
```

### 5. 图像操作 (`stdlib/images.js`)

```javascript
const images = {
    // 截图
    captureScreen: function() {
        let data = __native_screenshot();
        return new Image(data);
    },
    
    // 读取图片
    read: function(path) {
        let data = __native_read_image(path);
        return new Image(data);
    },
    
    // 保存图片
    save: function(img, path, format, quality) {
        format = format || "png";
        quality = quality || 100;
        return __native_save_image(img.data, path, format, quality);
    }
};

// 截图快捷函数
function captureScreen() {
    return images.captureScreen();
}

// 图像类
class Image {
    constructor(data) {
        this.data = data;
        this.width = data.width;
        this.height = data.height;
    }
    
    saveTo(path) {
        return images.save(this, path);
    }
    
    pixel(x, y) {
        return __native_get_pixel(this.data, x, y);
    }
}
```

---

## 迁移工具实现

### JS to Python转换器

```python
# flowx/converter/js_to_py.py

import re

def convert_autojs_to_python(js_code: str) -> str:
    """将AutoJS代码转换为FlowX Python代码"""
    
    py_code = js_code
    
    # 替换API调用
    replacements = {
        r'text\("([^"]+)"\)\.findOne\(\)\.click\(\)': 
            r'device.find(text="\1").click()',
        
        r'click\((\d+),\s*(\d+)\)': 
            r'device.click(\1, \2)',
        
        r'sleep\((\d+)\)': 
            r'time.sleep(\1 / 1000)',
        
        r'app\.launch\("([^"]+)"\)': 
            r'device.open_app("\1")',
    }
    
    for pattern, replacement in replacements.items():
        py_code = re.sub(pattern, replacement, py_code)
    
    # 添加导入
    py_code = "from flowx import Device\nimport time\n\ndevice = Device.android()\n\n" + py_code
    
    return py_code
```

---

## 验收标准

- [ ] 核心API实现完成（P0级别100%）
- [ ] 运行AutoJS示例脚本成功
- [ ] 迁移工具可用
- [ ] 提供迁移文档

---

## 测试脚本集

```javascript
// 测试1: 基础操作
click(100, 200);
sleep(1000);
swipe(100, 500, 100, 100, 300);

// 测试2: 元素查找
let btn = text("确认").findOne();
if (btn) {
    btn.click();
}

// 测试3: 应用操作
app.launch("com.tencent.mm");
sleep(2000);
let search = text("搜索").findOne();
search.click();
```

---

**创建日期**：2026-06-10  
**状态**：待开始
