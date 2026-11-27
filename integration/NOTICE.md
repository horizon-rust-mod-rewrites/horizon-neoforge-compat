# Licensing notice for the Horizon integration patch

`horizon-early-configuration.patch` contains modifications to Horizon and new
Horizon engine integration source/tests. It is distributed under
**GNU Lesser General Public License version 3 or (at your option) any later
version (LGPL-3.0-or-later)**, consistent with Horizon's pinned workspace license
and CONTRIBUTING.md. The compatibility repository's MIT license does not
relicense these Horizon-derived changes or the upstream context in the patch.

Upstream: https://github.com/rdbtCVS/horizon
Source revision: ae632edf48f6cae3ba52a5c81523ec712488c9cc
Original authors/copyright: the Horizon contributors, as recorded in the upstream
source history and notices. Preserve those attributions and upstream notices when
applying or redistributing. Integration additions were AI-assisted, 2026; no
upstream endorsement, authorship transfer or blanket MIT relicensing is implied.

Copies of the upstream LGPL-3.0 text and incorporated GPL-3.0 text are provided
unchanged under `LICENSES/`. The patch is editable corresponding modification
source; the exact original source pin is identified above. Recipients may
redistribute and modify the covered source under the LGPL terms. The covered
material is provided WITHOUT ANY WARRANTY, including MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the full license texts for terms and obligations,
including those applicable if distributing a combined executable.

The standalone Python application checker and patch-regeneration script are
original MIT-licensed development utilities under the repository's `LICENSE`.
The standalone companion `compat_bridge.rs` and `check_bridge.py` are original
MIT-licensed compatibility glue/utilities, not incorporated into the Horizon
patch. Combined executable distribution still needs applicable license review.

No NeoForge implementation source is copied in this patch. NeoForge is the
interoperability target; see the repository's NeoForge provenance and notices.
No Minecraft proprietary source, jars, resources, Create, Ponder or Sable source
or assets are included.
