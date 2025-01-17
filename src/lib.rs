// Copyright (C) 2023 Vince Vasta
// SPDX-License-Identifier: Apache-2.0
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

//! Data manipulation tool inspired by the [dplyr](https://dplyr.tidyverse.org/) grammar.
#![warn(clippy::all, rust_2018_idioms, missing_docs)]

pub mod interpreter;
pub mod repl;

mod completions;
mod config;
mod engine;
mod fuzzy;
mod parser;
mod signatures;
mod typing;
