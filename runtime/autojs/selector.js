// FlowX AutoJS Compatible Runtime - Selector Module

// Enhanced selector API
const selector = {
    text: function(str) {
        return new UiObject("text", str);
    },

    textContains: function(str) {
        return new UiObject("textContains", str);
    },

    textStartsWith: function(str) {
        return new UiObject("textStartsWith", str);
    },

    desc: function(str) {
        return new UiObject("desc", str);
    },

    id: function(str) {
        return new UiObject("id", str);
    },

    className: function(str) {
        return new UiObject("className", str);
    }
};

// UiObject class
function UiObject(type, value) {
    this.type = type;
    this.value = value;

    this.findOne = function(timeout) {
        timeout = timeout || 0;
        return __native.findElement({ type: this.type, value: this.value, timeout: timeout });
    };

    this.find = function() {
        return __native.findElements({ type: this.type, value: this.value });
    };

    this.exists = function() {
        return this.findOne() !== null;
    };

    this.click = function() {
        const element = this.findOne();
        if (element) {
            __native.click(element.bounds.centerX, element.bounds.centerY);
            return true;
        }
        return false;
    };

    this.waitFor = function(timeout) {
        timeout = timeout || 10000;
        const start = Date.now();
        while (Date.now() - start < timeout) {
            if (this.exists()) {
                return true;
            }
            sleep(100);
        }
        return false;
    };
}
