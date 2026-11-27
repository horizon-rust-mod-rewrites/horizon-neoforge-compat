# Reference material and licensing

The Rust code in this milestone is an original implementation of the bounded wire/lifecycle
infrastructure documented in WIRE.md, distributed under the included MIT license.
NeoForge's public source was read to establish field layout, not vendored here.
NeoForge source headers identify LGPL-2.1-only (NeoForged/Forge contributors).
Refer to the exact source artifact and per-file hashes in provenance/. This
notice is not a claim that unrelated upstream implementation code is MIT.

A Horizon-derived source patch is now included under integration/ with separate
LGPL-3.0-or-later licensing, notices and license texts; it is NOT relicensed MIT.
The original audit checkout remains unmodified. No Minecraft proprietary source,
game jars/assets, Create, Ponder or Sable implementation is redistributed.
Official MC classfiles were inspected and executed locally for bounded codec
reference tests, not included. Test preparation extracts/name-adapts selected
NeoForge codec declarations only in external scratch storage; generated upstream
Java source and proprietary game binaries are not redistributed. See
`docs/java-reference.md` and `provenance/setup-failure.md`.
Repository names identify interoperability targets, not endorsement.

Sable's supplied repository identifies PolyForm Shield 1.0.0. No Sable source
was copied or translated. Future adapters require separate legal review of
permitted protocol research and implementation; this archive grants no rights
to Sable. No automatic license-compatibility conclusion is implied.

The compiler/toolchain used to validate the source is not redistributed.
