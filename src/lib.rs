
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate gltf_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains extension specific data structures.
pub mod extensions;

/// Contains `Gltf`, and other related data structures.
pub mod gltf;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains functions for importing glTF 2.0 assets.
pub mod import;

/// Contains (de)serializable data structures that match the glTF JSON text.
pub mod json;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
pub mod mesh;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

/// Contains functions that validate glTF JSON data against the specification.
pub mod validation;

pub use self::gltf::{BufferData, Gltf, ImageData};
pub use self::import::StaticImporter;
