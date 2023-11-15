// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_json::json;

use console::{
    network::Network,
    prelude::{FromBytes, Parser, ToBytes},
    program::Register,
};

pub trait InstructionTrait<N: Network>: Clone + Parser + FromBytes + ToBytes {
    /// Returns the destination registers of the instruction.
    fn destinations(&self) -> Vec<Register<N>>;
    /// Returns `true` if the given name is a reserved opcode.
    fn is_reserved_opcode(name: &str) -> bool;
    /// ** Vanguard JSON serialization helper ** ///
    fn to_json(&self) -> serde_json::Value;
}