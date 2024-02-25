use std::collections::BTreeMap;

use crepedb::Bytes;

pub struct CrepeMemoryBackend {
    kv: BTreeMap<Bytes, BTreeMap<Bytes, Bytes>>,

    meta: BTreeMap<Bytes, Bytes>,
    snapshot: BTreeMap<Bytes, Bytes>,
    index: BTreeMap<Bytes, Bytes>,
}
