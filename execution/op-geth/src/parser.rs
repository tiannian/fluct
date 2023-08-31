use ethers_core::{
    types::{
        transaction::{
            eip1559::Eip1559RequestError,
            eip2930::Eip2930RequestError,
            optimism_deposited::{
                OptimismDepositedRequestError, OptimismDepositedTransactionRequest,
            },
            request::RequestError,
        },
        Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionRequest,
    },
    utils::rlp,
};
use fluct_core::{DepositedTransactionRequest, Parser, Transaction};
use thiserror::Error;

pub struct OpGethParser;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error(transparent)]
    RequestError(#[from] RequestError),

    #[error(transparent)]
    TypeDecodingError(#[from] rlp::DecoderError),

    #[error(transparent)]
    Eip2930Error(#[from] Eip2930RequestError),

    #[error(transparent)]
    Eip1559Error(#[from] Eip1559RequestError),

    #[error(transparent)]
    DepositedError(#[from] OptimismDepositedRequestError),

    #[error("Missing transaction body after type")]
    MissingTransactionPayload,

    #[error("Missing source_hash field in DepositedTransactionRequest")]
    MissingSourceHash,
}

impl Parser for OpGethParser {
    type Error = ParserError;

    fn serialize_transaction(tx: &Transaction) -> Vec<u8> {
        let mut encoded = vec![];

        match &tx {
            Transaction::Legacy(tx, signature) => {
                encoded.extend_from_slice(tx.rlp_signed(signature).as_ref());
            }
            Transaction::Eip2930(tx, signature) => {
                encoded.extend_from_slice(&[0x01]);
                encoded.extend_from_slice(tx.rlp_signed(signature).as_ref());
            }
            Transaction::Eip1559(tx, signature) => {
                encoded.extend_from_slice(&[0x02]);
                encoded.extend_from_slice(tx.rlp_signed(signature).as_ref());
            }
            Transaction::Deposited(tx) => {
                encoded.extend_from_slice(&[0x7F]);

                let mut r = rlp::RlpStream::new();
                r.begin_list(8);

                let system: Option<bool> = None;

                r.append(&tx.source_hash);
                rlp_opt(&mut r, &tx.tx.from);
                rlp_opt(&mut r, &tx.tx.to);
                r.append(&tx.mint);
                rlp_opt(&mut r, &tx.tx.value);
                rlp_opt(&mut r, &tx.tx.gas);
                rlp_opt(&mut r, &system);
                rlp_opt(&mut r, &tx.tx.data.as_deref());

                let data = r.out().freeze();

                encoded.extend_from_slice(data.as_ref());
            }
        }

        encoded
    }

    fn deserialize_transaction(bytes: &[u8]) -> Result<Transaction, Self::Error> {
        let r = rlp::Rlp::new(bytes);

        let data = r.data()?;

        let first = *data
            .first()
            .ok_or(rlp::DecoderError::Custom("empty slice"))?;

        if r.is_list() {
            // Legacy (0x00)
            // use the original rlp
            let decoded_request = TransactionRequest::decode_signed_rlp(&r)?;
            return Ok(Transaction::Legacy(decoded_request.0, decoded_request.1));
        }

        let rest = rlp::Rlp::new(
            r.as_raw()
                .get(1..)
                .ok_or(ParserError::MissingTransactionPayload)?,
        );

        if first == 0x01 {
            // EIP-2930 (0x01)
            let decoded_request = Eip2930TransactionRequest::decode_signed_rlp(&rest)?;
            return Ok(Transaction::Eip2930(decoded_request.0, decoded_request.1));
        }
        if first == 0x02 {
            // EIP-1559 (0x02)
            let decoded_request = Eip1559TransactionRequest::decode_signed_rlp(&rest)?;
            return Ok(Transaction::Eip1559(decoded_request.0, decoded_request.1));
        }

        if first == 0x7E {
            // Optimism Deposited (0x7E)
            let decoded_request = OptimismDepositedTransactionRequest::decode_signed_rlp(&rest)?;
            let request = decoded_request.0;
            return Ok(Transaction::Deposited(DepositedTransactionRequest {
                tx: request.tx,
                mint: request.mint.unwrap_or_default(),
                source_hash: request.source_hash.ok_or(ParserError::MissingSourceHash)?,
            }));
        }

        Err(rlp::DecoderError::Custom("invalid tx type").into())
    }
}

fn rlp_opt<T: rlp::Encodable>(rlp: &mut rlp::RlpStream, opt: &Option<T>) {
    if let Some(inner) = opt {
        rlp.append(inner);
    } else {
        rlp.append(&"");
    }
}
