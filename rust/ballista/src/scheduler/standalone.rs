// Copyright 2020 Andy Grove
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::Result;
use crate::scheduler::SchedulerClient;
use crate::serde::scheduler::ExecutorMeta;

use async_trait::async_trait;

pub struct StandaloneClient {}

#[async_trait]
impl SchedulerClient for StandaloneClient {
    async fn get_executors(&self) -> Result<Vec<ExecutorMeta>> {
        unimplemented!()
    }
}
