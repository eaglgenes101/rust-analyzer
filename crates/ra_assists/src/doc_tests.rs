//! Each assist definition has a special comment, which specifies docs and
//! example.
//!
//! We collect all the example and write the as tests in this module.

mod generated;

use ra_db::FileRange;
use test_utils::{assert_eq_text, extract_range_or_offset};

use crate::resolved_assists;

fn check(assist_id: &str, before: &str, after: &str) {
    let (selection, before) = extract_range_or_offset(before);
    let (db, file_id) = crate::helpers::with_single_file(&before);
    let frange = FileRange { file_id, range: selection.into() };

    let assist = resolved_assists(&db, frange)
        .into_iter()
        .find(|assist| assist.label.id.0 == assist_id)
        .unwrap_or_else(|| {
            panic!(
                "\n\nAssist is not applicable: {}\nAvailable assists: {}",
                assist_id,
                resolved_assists(&db, frange)
                    .into_iter()
                    .map(|assist| assist.label.id.0)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        });

    let actual = assist.action.edit.apply(&before);
    assert_eq_text!(after, &actual);
}
