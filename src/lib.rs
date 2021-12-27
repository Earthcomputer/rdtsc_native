use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use quanta::Clock;
use once_cell::sync::OnceCell;

static CLOCK: OnceCell<Clock> = OnceCell::new();

#[inline]
fn clock() -> &'static Clock {
    CLOCK.get_or_init(Clock::new)
}

#[no_mangle]
pub extern "system" fn Java_accuratetimer_AccurateTimer_initialize(_env: JNIEnv, _class: JClass) {
    clock(); // calibrate
}

#[no_mangle]
pub extern "system" fn Java_accuratetimer_AccurateTimer_rdtsc(_env: JNIEnv, _class: JClass) -> jlong {
    clock().raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_accuratetimer_AccurateTimer_delta(_env: JNIEnv, _class: JClass, start: jlong, end: jlong) -> jlong {
    clock().delta(start as u64, end as u64).as_nanos() as jlong
}
