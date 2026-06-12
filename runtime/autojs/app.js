// FlowX AutoJS Compatible Runtime - App Module

const app = {
    launch: function(package) {
        __native.launchApp(package);
    },

    launchApp: function(name) {
        __native.launchAppByName(name);
    },

    getAppName: function(package) {
        return __native.getAppName(package);
    },

    openAppSetting: function(package) {
        __native.openAppSetting(package);
    },

    getPackageName: function(appName) {
        return __native.getPackageName(appName);
    }
};
