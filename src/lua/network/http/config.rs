// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// For details, please see https://github.com/rusty-sec/lotus/
//
// Copyright (c) 2022 - Khaled Nassar
//
// Please note that this file was originally released under the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref REQUESTS_LIMIT: Arc<Mutex<i32>> = Arc::new(Mutex::new(5));
    pub static ref REQUESTS_SENT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    pub static ref SLEEP_TIME: Arc<Mutex<u64>> = Arc::new(Mutex::new(5));
    pub static ref VERBOSE_MODE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
