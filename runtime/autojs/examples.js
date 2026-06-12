// FlowX AutoJS Example Scripts
// These demonstrate the AutoJS-compatible API

// Example 1: Basic operations
function example1_basic() {
    // Click at coordinates
    click(100, 200);

    // Swipe gesture
    swipe(100, 200, 300, 400, 500);

    // Wait
    sleep(1000);

    // Show toast
    toast("Hello FlowX");

    // Log message
    log("Script started");
}

// Example 2: Element interaction
function example2_elements() {
    // Find and click by text
    text("登录").click();

    // Find and click by description
    desc("Login Button").click();

    // Find and click by ID
    id("com.example:id/button").click();

    // Check if element exists
    if (text("Welcome").exists()) {
        log("Welcome screen found");
    }
}

// Example 3: Complete automation
function example3_automation() {
    // Launch app
    app.launch("com.example.app");
    sleep(2000);

    // Wait for login screen
    text("登录").waitFor();

    // Input credentials
    id("username").click();
    sleep(500);
    // Note: text input would use native binding
    click(540, 600);

    // Click login button
    text("登录").click();
    sleep(3000);

    // Verify success
    if (text("Welcome").exists()) {
        toast("Login successful");
        log("Automation completed successfully");
    } else {
        toast("Login failed");
        log("Automation failed");
    }
}

// Example 4: Screenshot
function example4_screenshot() {
    // Take screenshot
    var img = screenshot();
    log("Screenshot captured");

    // Can be used for image recognition
    // findImage(img, template, threshold)
}

// Example 5: Advanced selectors
function example5_advanced() {
    // Text contains
    selector.textContains("用户").click();

    // Text starts with
    selector.textStartsWith("登录").click();

    // Class name
    selector.className("android.widget.Button").click();

    // Multiple elements
    var elements = selector.text("Item").find();
    log("Found " + elements.length + " items");

    // Wait for element
    if (selector.text("Loading").waitFor(10000)) {
        log("Element appeared");
    } else {
        log("Timeout waiting for element");
    }
}

// Run examples
log("FlowX AutoJS Examples");
log("===================");
