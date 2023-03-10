document.addEventListener('deviceready', onDeviceReady, false);

function onDeviceReady() {
    // Cordova is now initialized. Have fun!

    cordova.plugins.backgroundMode.enable();
    cordova.plugins.backgroundMode.setDefaults({ silent: true });

    (async () => {
        const { default: init, main } = await import("./psh_gui.js");
        init().then(() => {
            main();
        });
    })();
}
