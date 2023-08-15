#[cfg(not(target_os = "macos"))]
pub fn run<T, F: FnOnce() -> T + Send + 'static>(main: F) -> T
where
    T: Send + 'static,
{
    main()
}

#[cfg(target_os = "macos")]
pub fn run<T, F: FnOnce() -> T + Send + 'static>(main: F) -> T
where
    T: Send + 'static,
{
    use std::thread;

    use cocoa::appkit::NSApplication;
    use objc::{msg_send, sel, sel_impl};

    unsafe {
        let app = cocoa::appkit::NSApp();
        let t = thread::spawn(|| {
            

            let app = cocoa::appkit::NSApp();
            let res = main();
            let _: () = msg_send![app, terminate: cocoa::base::nil];

            res
        });

        app.run();

        t.join().unwrap()
    }
}