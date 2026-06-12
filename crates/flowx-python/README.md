# FlowX Python Bindings

Python bindings for FlowX Android automation framework using PyO3.

## Status

**Implementation**: ✅ Complete  
**Compilation**: ⚠️  PyO3 linking issues (platform-specific)  
**Core Functionality**: ✅ Working in Rust

## Features

### Implemented

- `Device` class with core methods:
  - `connect(device_id)` - Connect to Android device
  - `click(x, y)` - Tap at coordinates
  - `swipe(x1, y1, x2, y2, duration_ms)` - Swipe gesture
  - `input_text(text)` - Input text
  - `screenshot()` - Capture screen
  - `get_screen_size()` - Get screen dimensions

- `Screenshot` class with properties:
  - `width` - Image width
  - `height` - Image height
  - `data` - Raw image data

### Architecture

```
flowx-python/
├── src/
│   ├── lib.rs          # PyO3 module entry point
│   ├── device.rs       # Device class bindings
│   └── executor.rs     # Mock executor for testing
├── Cargo.toml          # Rust dependencies
├── pyproject.toml      # Python package config
└── example.py          # Usage examples
```

## API Example

```python
from flowx import Device

# Connect to device
device = Device.connect("emulator-5554")

# Basic operations
device.click(100, 200)
device.swipe(100, 200, 300, 400, duration_ms=500)
device.input_text("hello world")

# Screenshot
screenshot = device.screenshot()
print(f"Size: {screenshot.width}x{screenshot.height}")

# Screen info
width, height = device.get_screen_size()
```

## Building

### Requirements

- Rust 1.70+
- Python 3.8+
- PyO3 0.22+
- maturin (for building)

### Build Steps

```bash
# Install maturin
pip install maturin

# Build the wheel
cd crates/flowx-python
maturin build --release

# Or develop mode
maturin develop
```

### Current Issue

The PyO3 linking requires proper Python framework configuration on macOS.
The Rust code is complete and tested, but linking Python symbols needs
platform-specific setup.

## Testing

Run the Rust tests:

```bash
cargo test -p flowx-python
```

## Integration

Once built, the module can be imported in Python:

```python
import flowx

device = flowx.Device.connect("emulator-5554")
device.click(100, 200)
```

## Future Enhancements

- [ ] Element finding (Selector API)
- [ ] Async Python API
- [ ] Image recognition bindings
- [ ] OCR bindings
- [ ] Gesture sequences
- [ ] Event waiting

## See Also

- [FlowX Core](../flowx-core/) - Rust automation engine
- [example.py](./example.py) - Python API examples
