// Symphonia
// Copyright (c) 2019-2021 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

// The following lints are allowed in all Symphonia crates. Please see clippy.toml for their
// justification.
#![allow(clippy::comparison_chain)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::identity_op)]
#![allow(clippy::manual_range_contains)]

// Disable to better express the specification.
#![allow(clippy::collapsible_else_if)]

use symphonia_core::audio::{AudioBuffer, AudioBufferRef, AsAudioBufferRef};
use symphonia_core::codecs::{CODEC_TYPE_ALAC, CodecParameters, CodecDescriptor};
use symphonia_core::codecs::{Decoder, DecoderOptions, FinalizeResult};
use symphonia_core::errors::{Result, decode_error};
use symphonia_core::formats::Packet;
use symphonia_core::io::{ReadBitsRtl, BitReaderRtl};
use symphonia_core::support_codec;

/// Apple Lossless Audio Codec (ALAC) decoder.
pub struct AlacDecoder {
    /// Codec paramters.
    params: CodecParameters,
    /// Output buffer.
    buf: AudioBuffer<f32>,
}

impl Decoder for AlacDecoder {

    fn try_new(params: &CodecParameters, _: &DecoderOptions) -> Result<Self> {
        Ok(AlacDecoder {
            params: params.clone(),
            buf: AudioBuffer::unused(),
        })
    }

    fn reset(&mut self) {

    }

    fn supported_codecs() -> &'static [CodecDescriptor] {
        &[
            support_codec!(CODEC_TYPE_ALAC, "alac", "Apple Lossless Audio Codec"),
        ]
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.params
    }

    fn decode(&mut self, packet: &Packet) -> Result<AudioBufferRef<'_>> {
        let mut bs = BitReaderRtl::new(packet.buf());

        Ok(self.buf.as_audio_buffer_ref())
    }

    fn finalize(&mut self) -> FinalizeResult {
        Default::default()
    }
}
