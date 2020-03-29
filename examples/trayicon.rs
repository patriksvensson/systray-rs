#![windows_subsystem = "windows"]

#[cfg(not(target_os = "macos"))]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
    // w.set_tooltip(&"Whatever".to_string());
    app.set_icon_from_file("/usr/share/gxkb/flags/ua.png")?;

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(e) => panic!("Can't create tray icon app!"),
    }

    const ICON_BUFFER: &'static [u8] = include_bytes!("rust-logo.png");
    app.set_icon_from_buffer(ICON_BUFFER, 256, 256).unwrap();
    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;
    app.add_menu_item("Exit", |_| {
        Ok::<_, systray::Error>(())
    });
    app.wait_for_message()?;

    Ok(())
}
