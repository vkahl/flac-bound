use flac_sys::{FLAC__StreamEncoderInitStatus, FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_METADATA,
               FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED};
use std::convert::TryFrom;


/// Possible erroneous return values for the `FLAC__stream_encoder_init_*()` functions.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FlacEncoderInitError {
    /// General failure to set up encoder; call `FLAC__stream_encoder_get_state()` for cause.
    EncoderError = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR,

    /// The library was not compiled with support for the given container format.
    UnsupportedContainer = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER,

    /// A required callback was not supplied.
    InvalidCallbacks = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS,

    /// The encoder has an invalid setting for number of channels.
    InvalidNumberOfChannels = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS,

    /// The encoder has an invalid setting for bits-per-sample.
    ///
    /// FLAC supports 4-32 bps but the reference encoder currently supports only up to 24 bps.
    InvalidBitsPerSample = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE,

    /// The encoder has an invalid setting for the input sample rate.
    InvalidSampleRate = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE,

    /// The encoder has an invalid setting for the block size.
    InvalidBlockSize = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE,

    /// The encoder has an invalid setting for the maximum LPC order.
    InvalidMaxLpcOrder = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER,

    /// The encoder has an invalid setting for the precision of the quantized linear predictor coefficients.
    InvalidQlpCoeffPrecision = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION,

    /// The specified block size is less than the maximum LPC order.
    BlockSizeTooSmallForLpcOrder = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER,

    /// The encoder is bound to the <A HREF="../format.html#subset">Subset</A> but other settings violate it.
    NotStreamable = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE,

    /// The metadata input to the encoder is invalid, in one of the following ways:
    ///   * FLAC__stream_encoder_set_metadata() was called with a null pointer but a block count > 0
    ///   * One of the metadata blocks contains an undefined type
    ///   * It contains an illegal CUESHEET as checked by FLAC__format_cuesheet_is_legal()
    ///   * It contains an illegal SEEKTABLE as checked by FLAC__format_seektable_is_legal()
    ///   * It contains more than one SEEKTABLE block or more than one VORBIS_COMMENT block
    InvalidMetadata = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_METADATA,

    /// `FLAC__stream_encoder_init_*()` was called when the encoder was already initialized, usually because
    /// FLAC__stream_encoder_finish() was not called.
    AlreadyInitialized = FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED,
}

impl Into<FLAC__StreamEncoderInitStatus> for FlacEncoderInitError {
    fn into(self) -> FLAC__StreamEncoderInitStatus {
        self as FLAC__StreamEncoderInitStatus
    }
}

impl TryFrom<FLAC__StreamEncoderInitStatus> for FlacEncoderInitError {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(raw: FLAC__StreamEncoderInitStatus) -> Result<FlacEncoderInitError, ()> {
        Ok(match raw {
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ENCODER_ERROR => FlacEncoderInitError::EncoderError,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_UNSUPPORTED_CONTAINER => FlacEncoderInitError::UnsupportedContainer,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_CALLBACKS => FlacEncoderInitError::InvalidCallbacks,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_NUMBER_OF_CHANNELS => FlacEncoderInitError::InvalidNumberOfChannels,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BITS_PER_SAMPLE => FlacEncoderInitError::InvalidBitsPerSample,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_SAMPLE_RATE => FlacEncoderInitError::InvalidSampleRate,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_BLOCK_SIZE => FlacEncoderInitError::InvalidBlockSize,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_MAX_LPC_ORDER => FlacEncoderInitError::InvalidMaxLpcOrder,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_QLP_COEFF_PRECISION => FlacEncoderInitError::InvalidQlpCoeffPrecision,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_BLOCK_SIZE_TOO_SMALL_FOR_LPC_ORDER => {
                FlacEncoderInitError::BlockSizeTooSmallForLpcOrder
            }
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_NOT_STREAMABLE => FlacEncoderInitError::NotStreamable,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_INVALID_METADATA => FlacEncoderInitError::InvalidMetadata,
            FLAC__StreamEncoderInitStatus_FLAC__STREAM_ENCODER_INIT_STATUS_ALREADY_INITIALIZED => FlacEncoderInitError::AlreadyInitialized,
            _ => Err(())?,
        })
    }
}