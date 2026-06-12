// FlowX AutoJS Compatible Runtime - Global Functions
// This file provides AutoJS-compatible global functions

// Native bindings (injected by Rust)
const __native = {
    click: function(x, y) {},
    swipe: function(x1, y1, x2, y2, duration) {},
    sleep: function(ms) {},
    screenshot: function() {},
    findElement: function(selector) {},
};

// Global functions
function click(x, y) {
    __native.click(x, y);
}

function swipe(x1, y1, x2, y2, duration) {
    duration = duration || 300;
    __native.swipe(x1, y1, x2, y2, duration);
}

function sleep(ms) {
    __native.sleep(ms);
}

function screenshot() {
    return __native.screenshot();
}

function toast(msg) {
    console.log("[Toast] " + msg);
}

function log(msg) {
    console.log(msg);
}

// Selector functions
function text(str) {
    return new UiSelector("text", str);
}

function desc(str) {
    return new UiSelector("desc", str);
}

function id(str) {
    return new UiSelector("id", str);
}

// UiSelector class
function UiSelector(type, value) {
    this.type = type;
    this.value = value;

    this.click = function() {
        const element = __native.findElement({ type: this.type, value: this.value });
        if (element) {
            __native.click(element.x, element.y);
            return true;
        }
        return false;
    };

    this.exists = function() {
        const element = __native.findElement({ type: this.type, value: this.value });
        return element !== null;
    };
}
