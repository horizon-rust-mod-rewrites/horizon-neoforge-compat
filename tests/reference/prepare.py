#!/usr/bin/env python3
"""Prepare exact upstream codecs outside the checkout; no game/source redistribution.
Python 3.12+, a separately installed Java 21 JDK. See docs/java-reference.md.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import urllib.request
import zipfile

NF_URL = 'https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.248/neoforge-21.1.248-sources.jar'
NF_SHA256 = 'b4b912457549ff586873ce78b39b08af96f11a0e3a54c6b77576c6a25907083f'
SERVER_SHA1 = '59353fb40c36d304f2035d51e7d6e6baa98dc05c'
MAPPINGS_SHA1 = '03f8985492bda0afc0898465341eb0acef35f570'
ROOT = Path(__file__).resolve().parents[2]


def fetch(url, dest, digest=None, algorithm='sha1'):
    if not dest.exists():
        temp = dest.with_suffix(dest.suffix + '.partial')
        urllib.request.urlretrieve(url, temp)
        temp.replace(dest)
    data = dest.read_bytes()
    if digest and hashlib.new(algorithm, data).hexdigest() != digest:
        raise ValueError(f'checksum mismatch: {dest.name}; remove corrupt cache and retry')
    return data


def block(text, start):
    """Extract one balanced source block; exact pinned inputs only, not a parser."""
    i = text.index(start)
    opening = text.index('{', i)
    level = 1
    end = opening + 1
    while level:
        level += (text[end] == '{') - (text[end] == '}')
        end += 1
    return text[i:end]


def generate(scratch):
    manifest = json.loads((ROOT / 'provenance/modded-codecs.json').read_text())
    replacements = {
        'ConnectionProtocol': 'vu', 'FriendlyByteBuf': 'vw', 'ByteBufCodecs': 'yv',
        'StreamCodec': 'yx', 'PacketFlow': 'zh', 'ResourceLocation': 'akr',
        'yx.composite': 'yx.a', 'yv.map': 'yv.a', 'yv.idMapper': 'yv.a',
        'yv.collection': 'yv.a', 'yv.optional': 'yv.a', 'yv.STRING_UTF8': 'yv.l',
        'yv.BOOL': 'yv.b', 'akr.STREAM_CODEC': 'akr.b',
        'buf.readEnum': 'buf.b', 'buf.writeEnum': 'buf.a',
    }
    generated = scratch / 'generated'
    generated.mkdir(exist_ok=True)
    report = {'baseline': manifest['baseline'], 'transformations': replacements, 'sources': {}}
    with zipfile.ZipFile(scratch / 'neoforge-sources.jar') as archive:
        for path, expected in manifest['source_files'].items():
            data = archive.read(path)
            assert hashlib.sha256(data).hexdigest() == expected, path
            name = Path(path).stem
            if name == 'ModdedNetworkSetupFailedPayload':
                continue  # Owned by another workstream, deliberately excluded.
            text = data.decode()
            if name == 'NeoForgeStreamCodecs':
                # Copy the exact enumCodec method body, not a rewritten implementation.
                selected = 'public final class NeoForgeStreamCodecs {\n' + block(text, 'public static <B extends FriendlyByteBuf, V extends Enum<V>>') + '\n}'
            else:
                # Keep exact record declaration and exact STREAM_CODEC initializer.
                # Omit payload interface/ID/type and registry/negotiation helper methods;
                # none participates in these codec call graphs.
                start = text.index('public record ')
                opening = text.index('{', start)
                declaration = text[start:opening].split(' implements ')[0].rstrip()
                match = re.search(r'    public static (?:final )?StreamCodec<', text)
                assert match, name
                end = text.index(';', match.start()) + 1
                selected = declaration + ' {\n' + text[match.start():end] + '\n}'
            selected = text.split('package ', 1)[0] + 'import java.util.*;\n' + selected
            for old, new in replacements.items():
                selected = selected.replace(old, new)
            # NeoForgeStreamCodecs contains the token StreamCodec: restore its own name.
            selected = selected.replace('NeoForgeyxs', 'NeoForgeStreamCodecs')
            target = generated / (name + '.java')
            target.write_text(selected)
            report['sources'][path] = {'upstream_sha256': expected, 'adapted_sha256': hashlib.sha256(selected.encode()).hexdigest()}
    (scratch / 'source-adaptation.json').write_text(json.dumps(report, indent=2) + '\n')
    return generated


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--scratch', type=Path, required=True)
    parser.add_argument('--java-home', type=Path, required=True)
    args = parser.parse_args()
    scratch = args.scratch.resolve()
    if scratch == ROOT or ROOT in scratch.parents:
        parser.error('scratch must be outside the repository')
    scratch.mkdir(parents=True, exist_ok=True)
    fetch(NF_URL, scratch / 'neoforge-sources.jar', NF_SHA256, 'sha256')
    publisher = urllib.request.urlopen(NF_URL + '.sha256', timeout=90).read().decode().split()[0]
    assert publisher == NF_SHA256
    if not (scratch / 'version.json').exists():
        manifest = json.loads(fetch('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json', scratch / 'manifest.json'))
        version = next(v for v in manifest['versions'] if v['id'] == '1.21.1')
        fetch(version['url'], scratch / 'version.json', version['sha1'])
    metadata = json.loads((scratch / 'version.json').read_text())
    assert hashlib.sha256((scratch / 'version.json').read_bytes()).hexdigest() == 'bb0a1413744530df54451e4f973bac81b1e27477999e79cfe7e66aaf6d4db6e6'
    assert metadata['downloads']['server']['sha1'] == SERVER_SHA1
    assert metadata['downloads']['server_mappings']['sha1'] == MAPPINGS_SHA1
    fetch(metadata['downloads']['server_mappings']['url'], scratch / 'server_mappings.txt', MAPPINGS_SHA1)
    mapping_text = (scratch / 'server_mappings.txt').read_text()
    checks = {
        'net.minecraft.network.ConnectionProtocol -> vu:': [],
        'net.minecraft.network.protocol.PacketFlow -> zh:': [],
        'net.minecraft.network.FriendlyByteBuf -> vw:': ['readEnum(java.lang.Class) -> b', 'writeEnum(java.lang.Enum) -> a'],
        'net.minecraft.resources.ResourceLocation -> akr:': ['STREAM_CODEC -> b', 'parse(java.lang.String) -> a'],
        'net.minecraft.network.codec.StreamCodec -> yx:': ['composite(net.minecraft.network.codec.StreamCodec,java.util.function.Function,java.util.function.Function) -> a'],
        'net.minecraft.network.codec.ByteBufCodecs -> yv:': ['BOOL -> b', 'STRING_UTF8 -> l',
            'optional(net.minecraft.network.codec.StreamCodec) -> a',
            'collection(java.util.function.IntFunction,net.minecraft.network.codec.StreamCodec) -> a',
            'map(java.util.function.IntFunction,net.minecraft.network.codec.StreamCodec,net.minecraft.network.codec.StreamCodec) -> a',
            'idMapper(java.util.function.IntFunction,java.util.function.ToIntFunction) -> a'],
    }
    for header, members in checks.items():
        section = mapping_text[mapping_text.index(header):].split('\nnet.', 1)[0]
        for member in members:
            assert member in section, (header, member)
    if not (scratch / 'inner.jar').exists():
        fetch(metadata['downloads']['server']['url'], scratch / 'server.jar', SERVER_SHA1)
        with zipfile.ZipFile(scratch / 'server.jar') as archive:
            index = archive.read('META-INF/versions.list').decode().strip().split('\t')
            data = archive.read('META-INF/versions/' + index[2])
            assert hashlib.sha256(data).hexdigest() == index[0]
            (scratch / 'inner.jar').write_bytes(data)
    # Pin the extracted runtime too, even when reusing an external verified cache.
    assert hashlib.sha256((scratch / 'inner.jar').read_bytes()).hexdigest() == 'c301de10f575027d13eac18c7f34409d60648cf56a35d566aa1f530ff617840a'
    # Unsigned local copy: Java disallows signed official and unsigned test classes
    # sharing the obfuscated default package. No .class bytes are changed.
    with zipfile.ZipFile(scratch / 'inner.jar') as source, zipfile.ZipFile(scratch / 'runtime.jar', 'w', zipfile.ZIP_DEFLATED) as target:
        for entry in source.infolist():
            if entry.filename.upper().startswith('META-INF/') and entry.filename.upper().endswith(('.SF', '.RSA', '.DSA', '.EC')):
                continue
            target.writestr(entry, source.read(entry.filename))
    libraries = scratch / 'libraries'
    libraries.mkdir(exist_ok=True)
    # Headless codec call graph only; artifacts/versions taken from exact metadata.
    groups = {'com.google.code.gson', 'com.google.guava', 'com.mojang', 'io.netty', 'it.unimi.dsi', 'org.slf4j', 'org.apache.logging.log4j', 'org.apache.commons', 'org.joml'}
    for library in metadata['libraries']:
        if library['name'].split(':')[0] not in groups or 'native' in library['name']:
            continue
        artifact = library['downloads']['artifact']
        fetch(artifact['url'], libraries / Path(artifact['path']).name, artifact['sha1'])
    compiler_version = subprocess.run([str(args.java_home / 'bin/javac'), '-version'], capture_output=True, text=True, check=True)
    assert (compiler_version.stdout + compiler_version.stderr).startswith('javac 21.'), 'Java 21 JDK required'
    generated = generate(scratch)
    classes = scratch / 'classes'
    classes.mkdir(exist_ok=True)
    cp = f'{scratch / "runtime.jar"}:{libraries}/*'
    command = [str(args.java_home / 'bin/javac'), '-proc:none', '-encoding', 'UTF-8', '-cp', cp, '-d', str(classes)]
    subprocess.run(command + [str(p) for p in sorted(generated.glob('*.java'))] + [str(Path(__file__).with_name('CodecReference.java'))], check=True, cwd=scratch)
    print('Compiled source-extracted NeoForge query/setup codecs against official MC 1.21.1.')
    print('Classpath:', f'{classes}:{cp}')


if __name__ == '__main__':
    main()
