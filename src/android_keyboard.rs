// Android keyboard handling module
// Based on: https://github.com/rust-mobile/android-activity/pull/178

#[cfg(target_os = "android")]
use std::sync::OnceLock;

// Store the Android app context - we'll need to find a way to initialize this
#[cfg(target_os = "android")]
static ANDROID_APP_VM: OnceLock<(*mut ndk_sys::JavaVM, *mut ndk_sys::jobject)> = OnceLock::new();

#[cfg(target_os = "android")]
pub fn set_android_app_from_raw(vm: *mut ndk_sys::JavaVM, activity: *mut ndk_sys::jobject) {
    let _ = ANDROID_APP_VM.set((vm, activity));
}

/// Show the Android soft keyboard
/// This implementation is based on the working solution from wareya's comment
#[cfg(target_os = "android")]
pub fn show_soft_input(show_implicit: bool) -> Result<(), Box<dyn std::error::Error>> {
    use jni::objects::JValue;

    let (vm_ptr, activity_ptr) = match ANDROID_APP_VM.get() {
        Some((vm, activity)) => (*vm, *activity),
        None => {
            log::warn!("Android app context not initialized");
            return Ok(()); // Fail silently for now
        }
    };

    // Create JVM and activity objects from raw pointers
    let jvm = unsafe { jni::JavaVM::from_raw(vm_ptr)? };
    let activity = unsafe { jni::objects::JObject::from_raw(activity_ptr) };

    let mut env = jvm.attach_current_thread()?;

    // Early exception check
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("JNI exception during initialization".into());
    }

    // Get InputMethodManager
    let class_ctxt = env.find_class("android/content/Context")?;
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("Failed to find Context class".into());
    }

    let ims = env.get_static_field(class_ctxt, "INPUT_METHOD_SERVICE", "Ljava/lang/String;")?;
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("Failed to get INPUT_METHOD_SERVICE".into());
    }

    let im_manager = env
        .call_method(
            &activity,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[(&ims).into()],
        )?
        .l()?;
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("Failed to get InputMethodManager".into());
    }

    // Get the window and its decor view
    let jni_window = env
        .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("Failed to get window".into());
    }

    let view = env
        .call_method(&jni_window, "getDecorView", "()Landroid/view/View;", &[])?
        .l()?;
    if env.exception_check()? {
        env.exception_clear()?;
        return Err("Failed to get decor view".into());
    }

    // Show the soft input
    let flags = if show_implicit {
        ndk_sys::ANATIVEACTIVITY_SHOW_SOFT_INPUT_IMPLICIT as i32
    } else {
        0i32
    };

    env.call_method(
        im_manager,
        "showSoftInput",
        "(Landroid/view/View;I)Z",
        &[JValue::Object(&view), JValue::Int(flags)],
    )?;

    // showSoftInput can trigger exceptions if the keyboard is currently animating
    if env.exception_check()? {
        env.exception_clear()?;
    }

    log::debug!("Android soft keyboard show command sent");
    Ok(())
}

/// Hide the Android soft keyboard
#[cfg(target_os = "android")]
pub fn hide_soft_input() -> Result<(), Box<dyn std::error::Error>> {
    use jni::objects::JValue;

    let (vm_ptr, activity_ptr) = match ANDROID_APP_VM.get() {
        Some((vm, activity)) => (*vm, *activity),
        None => {
            log::warn!("Android app context not initialized");
            return Ok(()); // Fail silently for now
        }
    };

    // Create JVM and activity objects from raw pointers
    let jvm = unsafe { jni::JavaVM::from_raw(vm_ptr)? };
    let activity = unsafe { jni::objects::JObject::from_raw(activity_ptr) };

    let mut env = jvm.attach_current_thread()?;

    // Get InputMethodManager
    let class_ctxt = env.find_class("android/content/Context")?;
    let ims = env.get_static_field(class_ctxt, "INPUT_METHOD_SERVICE", "Ljava/lang/String;")?;

    let im_manager = env
        .call_method(
            &activity,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[(&ims).into()],
        )?
        .l()?;

    // Get the window and its decor view to get the window token
    let window = env
        .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])?
        .l()?;
    let decor_view = env
        .call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?
        .l()?;
    let window_token = env
        .call_method(&decor_view, "getWindowToken", "()Landroid/os/IBinder;", &[])?
        .l()?;

    // Hide the soft input
    env.call_method(
        &im_manager,
        "hideSoftInputFromWindow",
        "(Landroid/os/IBinder;I)Z",
        &[
            JValue::Object(&window_token),
            JValue::Int(0), // flags, usually 0
        ],
    )?;

    // Clear any exceptions that might have occurred
    if env.exception_check()? {
        env.exception_clear()?;
    }

    log::debug!("Android soft keyboard hide command sent");
    Ok(())
}

// Simplified approach: check if any text field has focus
pub fn check_ui_keyboard_focus(ctx: &egui::Context) -> bool {
    ctx.memory(|mem| mem.focused()).is_some()
}

// Alternative function to initialize from eframe/winit if available
#[cfg(target_os = "android")]
pub fn try_init_from_eframe() {
    // This is a placeholder - we'll need to find the right way to get
    // the Android context from eframe when running on Android
    log::info!("Attempting to initialize Android context from eframe");
}

// Stub functions for non-Android platforms
#[cfg(not(target_os = "android"))]
pub fn set_android_app(_app: ()) {
    // No-op on non-Android platforms
}

#[cfg(not(target_os = "android"))]
pub fn set_android_app_from_raw(_vm: *mut std::ffi::c_void, _activity: *mut std::ffi::c_void) {
    // No-op on non-Android platforms
}

#[cfg(not(target_os = "android"))]
pub fn try_init_from_eframe() {
    // No-op on non-Android platforms
}

#[cfg(not(target_os = "android"))]
pub fn show_soft_input(_show_implicit: bool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(()) // No-op on non-Android platforms
}

#[cfg(not(target_os = "android"))]
pub fn hide_soft_input() -> Result<(), Box<dyn std::error::Error>> {
    Ok(()) // No-op on non-Android platforms
}
