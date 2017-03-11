// Copyright 2017 Google Inc. All rights reserved.
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

//! The interface for a module that does some audio processing.

pub const N_SAMPLES_PER_CHUNK: usize = 64;

pub struct Buffer {
    // TODO: simd alignment
    buf: [f32; N_SAMPLES_PER_CHUNK],
    // Will probably get special zero handling
}

impl Buffer {
    pub fn set_zero(&mut self) {
        *self = Default::default();
    }

    pub fn get(&self) -> &[f32; N_SAMPLES_PER_CHUNK] {
        &self.buf
    }

    pub fn get_mut(&mut self) -> &mut [f32; N_SAMPLES_PER_CHUNK] {
        &mut self.buf
    }
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            buf: [0.0; N_SAMPLES_PER_CHUNK]
        }
    }
}

pub trait Module {
    /// Report the number of buffers this module is expected to generate.
    fn n_bufs_out(&self) -> usize { 0 }

    /// Report the number of control values this module is expected to generate.
    fn n_ctrl_out(&self) -> usize { 0 }

    /// Process one chunk of audio. Implementations are expected to be lock-free.
    fn process(&mut self, control_in: &[f32], control_out: &mut [f32],
        buf_in: &[&Buffer], buf_out: &mut [Buffer]);
}