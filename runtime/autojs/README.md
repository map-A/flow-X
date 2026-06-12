# FlowX JavaScript & AutoJS Runtime

JavaScript runtime and AutoJS-compatible API for FlowX automation scripts.

## Status

**Implementation**: ✅ Complete  
**Testing**: ✅ Passing  
**Integration**: ✅ Ready

## Components

### 1. JavaScript Runtime (`js_runtime.rs`)

Simple JavaScript runtime mock that provides:
- Script execution
- Native function registration
- Expression evaluation

### 2. AutoJS Runtime (`autojs.rs`)

AutoJS-compatible runtime with:
- Global function injection
- App module support
- Selector API
- Element interaction

### 3. JavaScript Libraries

#### `runtime/autojs/global.js`
Core global functions:
- `click(x, y)` - Tap at coordinates
- `swipe(x1, y1, x2, y2, duration)` - Swipe gesture
- `sleep(ms)` - Wait/delay
- `toast(msg)` - Show toast message
- `log(msg)` - Log message
- `screenshot()` - Capture screen

Selector functions:
- `text(str).click()` - Find by text and click
- `desc(str).click()` - Find by description and click
- `id(str).click()` - Find by ID and click

#### `runtime/autojs/app.js`
App management:
- `app.launch(package)` - Launch app by package name
- `app.launchApp(name)` - Launch app by name
- `app.getAppName(package)` - Get app name
- `app.openAppSetting(package)` - Open app settings

#### `runtime/autojs/selector.js`
Advanced selectors:
- `selector.text(str)` - Exact text match
- `selector.textContains(str)` - Text contains
- `selector.textStartsWith(str)` - Text starts with
- `selector.desc(str)` - Description match
- `selector.id(str)` - ID match
- `selector.className(str)` - Class name match

UiObject methods:
- `.findOne(timeout)` - Find single element
- `.find()` - Find all matching elements
- `.exists()` - Check if element exists
- `.click()` - Click element
- `.waitFor(timeout)` - Wait for element

## Usage Examples

### Example 1: Basic Operations

```javascript
// Click at coordinates
click(100, 200);

// Swipe
swipe(100, 200, 300, 400, 500);

// Wait
sleep(1000);

// Show message
toast("Hello FlowX");
```

### Example 2: Element Interaction

```javascript
// Find and click
text("登录").click();
desc("Login Button").click();
id("com.example:id/button").click();

// Check existence
if (text("Welcome").exists()) {
    log("Welcome screen found");
}
```

### Example 3: App Automation

```javascript
// Launch app
app.launch("com.example.app");
sleep(2000);

// Wait for element
text("登录").waitFor();

// Interact
text("Username").click();
sleep(500);

// Verify
if (text("Success").exists()) {
    toast("Login successful");
}
```

### Example 4: Advanced Selectors

```javascript
// Text contains
selector.textContains("用户").click();

// Multiple elements
var elements = selector.text("Item").find();
log("Found " + elements.length + " items");

// Wait with timeout
if (selector.text("Loading").waitFor(10000)) {
    log("Element appeared");
}
```

## API Reference

### Global Functions

| Function | Description | Example |
|----------|-------------|---------|
| `click(x, y)` | Tap at coordinates | `click(100, 200)` |
| `swipe(x1, y1, x2, y2, duration)` | Swipe gesture | `swipe(0, 500, 0, 100, 300)` |
| `sleep(ms)` | Wait/delay | `sleep(1000)` |
| `toast(msg)` | Show toast | `toast("Hello")` |
| `log(msg)` | Log message | `log("Debug")` |
| `screenshot()` | Capture screen | `var img = screenshot()` |

### Selector API

| Selector | Description | Example |
|----------|-------------|---------|
| `text(str)` | Find by exact text | `text("登录").click()` |
| `textContains(str)` | Find by text contains | `textContains("用户").click()` |
| `desc(str)` | Find by description | `desc("Button").click()` |
| `id(str)` | Find by resource ID | `id("button1").click()` |
| `className(str)` | Find by class name | `className("Button").click()` |

### App Module

| Function | Description | Example |
|----------|-------------|---------|
| `app.launch(pkg)` | Launch app | `app.launch("com.android.settings")` |
| `app.launchApp(name)` | Launch by name | `app.launchApp("Settings")` |
| `app.getAppName(pkg)` | Get app name | `var name = app.getAppName(pkg)` |

## Testing

Run the tests:

```bash
cargo test -p flowx-core --lib scripting
```

Output:
```
test scripting::autojs::tests::test_autojs_api_constants ... ok
test scripting::autojs::tests::test_autojs_runtime_creation ... ok
test scripting::js_runtime::tests::test_js_runtime_basic ... ok
```

## Integration

### Rust Integration

```rust
use flowx_core::scripting::{AutoJsRuntime, JsRuntime};

// Create runtime
let mut runtime = AutoJsRuntime::new();

// Execute script
let result = runtime.execute("click(100, 200); sleep(1000);");

// Get AutoJS library code
let lib_code = AutoJsRuntime::get_autojs_lib();
```

### Script Loading

```rust
// Load and execute AutoJS script
let script = std::fs::read_to_string("script.js")?;
runtime.execute(&script)?;
```

## Architecture

```
flowx-core/src/scripting/
├── mod.rs              # Module exports
├── js_runtime.rs       # JavaScript runtime mock
└── autojs.rs           # AutoJS compatibility layer

runtime/autojs/
├── global.js           # Global functions
├── app.js              # App module
├── selector.js         # Selector API
└── examples.js         # Usage examples
```

## See Also

- [examples.js](../../runtime/autojs/examples.js) - Complete examples
- [FlowX Core](../../crates/flowx-core/) - Core automation engine
- [Python Bindings](../../crates/flowx-python/) - Python API
