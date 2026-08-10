pub mod objects;

use ::jni::{JNIEnv, JavaVM, NativeMethod, objects::JObject};
use jni::{objects::JString, sys::jboolean};
use once_cell::sync::OnceCell;
use std::ffi::c_void;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    if let Ok(()) = GLOBAL_JVM.set(env.get_java_vm()?) {
        #[cfg(target_os = "android")]
        {
            env.register_native_methods(
                "com/nonpolynomial/btleplug/android/impl/Adapter",
                &[
                    NativeMethod {
                        name: "reportScanResult".into(),
                        sig: "(Landroid/bluetooth/le/ScanResult;)V".into(),
                        fn_ptr: adapter_report_scan_result as *mut c_void,
                    },
                    NativeMethod {
                        name: "onConnectionStateChanged".into(),
                        sig: "(Ljava/lang/String;Z)V".into(),
                        fn_ptr: adapter_on_connection_state_changed as *mut c_void,
                    },
                ],
            )?;
        }
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/Peripheral",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/ScanFilter",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/NotConnectedException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/PermissionDeniedException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/UnexpectedCallbackException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/UnexpectedCharacteristicException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/NoSuchCharacteristicException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "com/nonpolynomial/btleplug/android/impl/NoBluetoothAdapterException",
        )?;

        // jni-utils class caching
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/future/Future",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/future/FutureException",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/ops/FnAdapter",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/stream/Stream",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/stream/StreamPoll",
        )?;
        super::jni_utils::classcache::find_add_class(env, "io/github/gedgygedgy/rust/task/Waker")?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/task/PollResult",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/ops/FnRunnableImpl",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/ops/FnBiFunctionImpl",
        )?;
        super::jni_utils::classcache::find_add_class(
            env,
            "io/github/gedgygedgy/rust/ops/FnFunctionImpl",
        )?;

        // FnAdapter native method registration
        let fn_adapter_class =
            env.auto_local(env.find_class("io/github/gedgygedgy/rust/ops/FnAdapter")?);
        env.register_native_methods(
            &fn_adapter_class,
            &[
                NativeMethod {
                    name: "callInternal".into(),
                    sig:
                        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"
                            .into(),
                    fn_ptr: super::jni_utils::ops::fn_adapter_call_internal as *mut c_void,
                },
                NativeMethod {
                    name: "closeInternal".into(),
                    sig: "()V".into(),
                    fn_ptr: super::jni_utils::ops::fn_adapter_close_internal as *mut c_void,
                },
            ],
        )?;
    }
    Ok(())
}

pub fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect(
        "Droidplug has not been initialized. Please initialize it with btleplug::platform::init().",
    )
}

pub(crate) fn get_env() -> Result<JNIEnv<'static>, ::jni::errors::Error> {
    log::trace!("[btleplug] get_env called");
    match global_jvm().get_env() {
        Ok(env) => {
            log::trace!("[btleplug] get_env: attached OK");
            Ok(env)
        }
        Err(e) => {
            log::warn!("[btleplug] get_env: thread detached, attaching permanently. err={e:?}");
            match global_jvm().attach_current_thread_permanently() {
                Ok(env) => {
                    log::info!("[btleplug] get_env: attached permanently");
                    Ok(env)
                }
                Err(e2) => {
                    log::error!("[btleplug] get_env: attach_current_thread_permanently failed: {e2:?}");
                    Err(e2)
                }
            }
        }
    }
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

#[cfg(target_os = "android")]
extern "C" fn adapter_report_scan_result(env: JNIEnv, obj: JObject, scan_result: JObject) {
    let _ = super::adapter::adapter_report_scan_result_internal(&env, obj, scan_result);
}

#[cfg(target_os = "android")]
extern "C" fn adapter_on_connection_state_changed(
    env: JNIEnv,
    obj: JObject,
    addr: JString,
    connected: jboolean,
) {
    let _ =
        super::adapter::adapter_on_connection_state_changed_internal(&env, obj, addr, connected);
}
