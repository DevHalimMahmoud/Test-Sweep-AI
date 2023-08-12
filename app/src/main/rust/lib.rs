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
    let response = match get("https://example.com") {
        Ok(res) => match res.text() {
            Ok(text) => text,
            Err(_) => return env.new_string("Error getting text from response").unwrap().into_inner(),
        },
        Err(_) => return env.new_string("Error making network request").unwrap().into_inner(),
    };
    let output = match env.new_string(response) {
        Ok(string) => string,
        Err(_) => return env.new_string("Error creating new string").unwrap().into_inner(),
    };
    output.into_inner()
}