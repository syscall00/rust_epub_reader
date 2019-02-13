// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Utilities, macOS specific.

use cocoa::base::{BOOL, YES};

pub fn init() {}

/// Panic if not on the main thread.assert_main_thread()
///
/// Many Cocoa operations are only valid on the main thread, and (I think)
/// undefined behavior is possible if invoked from other threads. If so,
/// failing on non main thread is necessary for safety.
pub fn assert_main_thread() {
    unsafe {
        let is_main_thread: BOOL = msg_send!(class!(NSThread), isMainThread);
        assert_eq!(is_main_thread, YES);
    }
}
