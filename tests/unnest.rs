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
use anyhow::Result;
use indoc::indoc;

use dply::interpreter;

#[test]
fn unnest_ints() -> Result<()> {
    let input = indoc! {r#"
        parquet("tests/data/lists.parquet") |
            mutate(ints_len = len(ints)) |
            relocate(ints_len, ints, after = shape_id) |
            select(shape_id, ints_len, ints) |
            unnest(ints) |
            head()
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            shape: (10, 3)
            shape_id|ints_len|ints
            u32|i32|u32
            ---
            1|3|3
            1|3|88
            1|3|94
            2|1|73
            3|null|null
            4|2|43
            4|2|97
            5|null|null
            6|1|65
            7|4|1
            ---
       "#
        )
    );

    Ok(())
}

#[test]
fn unnest_str() -> Result<()> {
    let input = indoc! {r#"
        parquet("tests/data/lists.parquet") |
            mutate(tags_len = len(tags)) |
            relocate(tags_len, tags, after = shape_id) |
            select(shape_id, tags_len, tags) |
            unnest(tags) |
            head()
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            shape: (10, 3)
            shape_id|tags_len|tags
            u32|i32|str
            ---
            1|4|tag2
            1|4|tag5
            1|4|tag8
            1|4|tag8
            2|1|tag9
            3|1|tag5
            4|1|tag7
            5|3|tag2
            5|3|tag3
            5|3|tag4
            ---
       "#
        )
    );

    Ok(())
}

#[test]
fn unnest_floats() -> Result<()> {
    let input = indoc! {r#"
        parquet("tests/data/lists.parquet") |
            mutate(floats_len = len(floats)) |
            relocate(floats_len, floats, after = shape_id) |
            select(shape_id, floats_len, floats) |
            unnest(floats) |
            head(12)
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            shape: (12, 3)
            shape_id|floats_len|floats
            u32|i32|f64
            ---
            1|4|2.5
            1|4|3.5
            1|4|6.0
            1|4|23.0
            2|3|3.5
            2|3|15.0
            2|3|23.0
            3|4|1.0
            3|4|2.5
            3|4|6.0
            3|4|6.0
            4|4|2.5
            ---
       "#
        )
    );

    Ok(())
}

#[test]
#[ignore = "need unnest structs"]
fn unnest_structs() -> Result<()> {
    let input = indoc! {r#"
        parquet("tests/data/structs.parquet") |
            unnest(points) |
            head()
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            shape: (10, 2)
            shape_id|points
            u32|struct[4]
            ---
            1|{"s1",0,-7.144482,-2.752852}
            1|{"s1",1,-3.377404,-2.862458}
            1|{"s1",2,-4.05302,6.336014}
            2|{null,null,null,null}
            3|{"s3",0,-8.744724,-0.039072}
            4|{"s4",0,-0.807573,-7.81899}
            5|{"s5",0,-2.831063,5.288568}
            6|{"s6",0,4.039896,-3.030655}
            7|{"s7",0,4.160488,9.694407}
            7|{"s7",1,-7.926216,-4.505739}
            ---
       "#
        )
    );

    // Unnest twice to extract the struct fields.
    let input = indoc! {r#"
        parquet("tests/data/structs.parquet") |
            unnest(points, points) |
            head()
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            shape: (10, 5)
            shape_id|ptag|pid|x|y
            u32|str|i32|f32|f32
            ---
            1|s1|0|-7.144482|-2.752852
            1|s1|1|-3.377404|-2.862458
            1|s1|2|-4.05302|6.336014
            2|null|null|null|null
            3|s3|0|-8.744724|-0.039072
            4|s4|0|-0.807573|-7.81899
            5|s5|0|-2.831063|5.288568
            6|s6|0|4.039896|-3.030655
            7|s7|0|4.160488|9.694407
            7|s7|1|-7.926216|-4.505739
            ---
       "#
        )
    );

    Ok(())
}
