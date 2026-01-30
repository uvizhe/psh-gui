document.addEventListener('deviceready', onDeviceReady, false);

function onDeviceReady() {
    (async () => {
        const { default: init, main } = await import("./psh_gui.js");
        init().then(() => {
            main();
        });
    })();
}
