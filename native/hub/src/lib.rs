use rinf::{dart_shutdown, write_interface};
use tokio::spawn;

mod modules;
use crate::modules::create_actors;

// Magic function to initialize the android context
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint {
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}

write_interface!();

#[tokio::main(flavor = "current_thread")]
async fn main() {
    spawn(create_actors());
    dart_shutdown().await;
}
