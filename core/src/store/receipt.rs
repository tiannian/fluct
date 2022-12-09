/// Receipt store
pub struct ReceiptStore<KV> {
    /// receipthash -> receipt
    receipt: KV,

    /// receipt hash -> txhash
    receipt_tx: KV,

    /// topic -> receipthash
    log_index: KV,
}
