use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamNoteOutput {
    pub stream_notes: model::note::Model,
}
