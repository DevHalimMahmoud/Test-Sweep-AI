extern crate jni;
extern crate reqwest;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use reqwest::blocking::get;

#[no_mangle]
pub extern "system" fn Java_com_abdelhalim_testsweepai_MainActivity_makeNetworkRequest(
    env: JNIEnv,
    _: JClass,
) -> jstring {
    let response = get("https://example.com").unwrap().text().unwrap();
    let output = env.new_string(response).unwrap();
    output.into_inner()
}