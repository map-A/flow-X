"""
FlowX Python Bindings - Example Usage

This file demonstrates the planned Python API for FlowX.
Currently the binding compilation has linking issues with PyO3,
but the Rust implementation is complete and ready.

Example Usage:
"""

# Example 1: Basic device operations
def example_basic():
    from flowx import Device

    device = Device.connect("emulator-5554")

    # Click at coordinates
    device.click(100, 200)

    # Swipe gesture
    device.swipe(100, 200, 300, 400)
    device.swipe(100, 200, 300, 400, duration_ms=500)

    # Input text
    device.input_text("hello world")

    # Take screenshot
    screenshot = device.screenshot()
    print(f"Screenshot: {screenshot.width}x{screenshot.height}")
    print(f"Data size: {len(screenshot.data)} bytes")

    # Get screen size
    width, height = device.get_screen_size()
    print(f"Screen size: {width}x{height}")


# Example 2: Element finding (planned)
def example_elements():
    from flowx import Device, Selector

    device = Device.connect("emulator-5554")

    # Find element by text
    element = device.find_element(Selector.text("登录"))
    element.click()

    # Find element by ID
    element = device.find_element(Selector.id("com.example:id/button"))
    element.click()


# Example 3: Automation script
def example_automation():
    from flowx import Device

    device = Device.connect("emulator-5554")

    # Open app
    device.open_app("com.example.app")

    # Wait and click
    import time
    time.sleep(2)
    device.click(540, 960)

    # Input credentials
    device.click(540, 600)  # Username field
    device.input_text("user@example.com")

    device.click(540, 800)  # Password field
    device.input_text("password123")

    # Submit
    device.click(540, 1000)  # Login button


if __name__ == "__main__":
    print("FlowX Python Bindings Example")
    print("=" * 50)
    print()
    print("Note: This is a demonstration of the planned API.")
    print("The Rust implementation is complete, but PyO3 linking")
    print("requires additional configuration for your system.")
    print()
    print("API Methods:")
    print("  - Device.connect(device_id) -> Device")
    print("  - device.click(x, y)")
    print("  - device.swipe(x1, y1, x2, y2, duration_ms?)")
    print("  - device.input_text(text)")
    print("  - device.screenshot() -> Screenshot")
    print("  - device.get_screen_size() -> (width, height)")
