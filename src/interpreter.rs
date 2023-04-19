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

//! Interpreter for dply expressions.
use anyhow::Result;

use crate::{parser, typing};

/// Evaluates a dply script.
pub fn eval(input: &str) -> Result<()> {
    let exprs = parser::parse(input)?;

    for expr in &exprs {
        typing::pipeline(expr)?;
    }

    for expr in exprs {
        println!("{expr}\n");
    }

    Ok(())
}
