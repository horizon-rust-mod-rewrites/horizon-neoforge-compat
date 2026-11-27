/* Test driver only. Wire operations are delegated to extracted upstream codecs.
 * Default package is necessary to access Mojang's obfuscated default-package classes.
 * This file contains no Minecraft or NeoForge implementation code.
 */
import io.netty.buffer.Unpooled;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;

public final class CodecReference {
    private static final HexFormat HEX = HexFormat.of();
    private static String string(String hex) {
        return new String(HEX.parseHex(hex), StandardCharsets.UTF_8);
    }
    private static String hex(String value) {
        return HEX.formatHex(value.getBytes(StandardCharsets.UTF_8));
    }
    private static Object model(String kind, String text) {
        Map<vu, Set<ModdedNetworkQueryComponent>> query = new IdentityHashMap<>();
        Map<vu, Map<akr, NetworkChannel>> setup = new IdentityHashMap<>();
        vu phase = null;
        if (!text.isEmpty()) for (String line : text.split(";", -1)) {
            String[] fields = line.split(":", -1);
            if (fields[0].equals("P")) {
                phase = vu.values()[Integer.parseInt(fields[1])];
                if (kind.equals("query")) query.put(phase, new HashSet<>());
                else setup.put(phase, new HashMap<>());
            } else {
                Objects.requireNonNull(phase);
                akr id = akr.a(string(fields[1]));
                if (kind.equals("query")) {
                    int flow = Integer.parseInt(fields[3]);
                    query.get(phase).add(new ModdedNetworkQueryComponent(id, string(fields[2]),
                        flow < 0 ? Optional.empty() : Optional.of(zh.values()[flow]), fields[4].equals("1")));
                } else {
                    setup.get(phase).put(id, new NetworkChannel(id, string(fields[2])));
                }
            }
        }
        return kind.equals("query") ? new ModdedNetworkQueryPayload(query)
            : new ModdedNetworkPayload(new NetworkPayloadSetup(setup));
    }
    private static String canonical(String kind, Object object) {
        List<String> result = new ArrayList<>();
        for (vu phase : vu.values()) {
            if (kind.equals("query")) {
                Set<ModdedNetworkQueryComponent> values = ((ModdedNetworkQueryPayload)object).queries().get(phase);
                if (values == null) continue;
                result.add("P:" + phase.ordinal());
                values.stream().sorted(Comparator.comparing(v -> v.id().toString())).forEach(v ->
                    result.add("Q:" + hex(v.id().toString()) + ":" + hex(v.version()) + ":"
                        + v.flow().map(Enum::ordinal).orElse(-1) + ":" + (v.optional() ? 1 : 0)));
            } else {
                Map<akr, NetworkChannel> values = ((ModdedNetworkPayload)object).setup().channels().get(phase);
                if (values == null) continue;
                result.add("P:" + phase.ordinal());
                // Include both IDs: mismatch is observable, never silently normalized.
                values.entrySet().stream().sorted(Comparator.comparing(e -> e.getKey().toString())).forEach(e -> {
                    NetworkChannel v = e.getValue();
                    result.add(e.getKey().equals(v.id())
                        ? "S:" + hex(v.id().toString()) + ":" + hex(v.chosenVersion())
                        : "M:" + hex(e.getKey().toString()) + ":" + hex(v.id().toString()) + ":" + hex(v.chosenVersion()));
                });
            }
        }
        return String.join(";", result);
    }
    @SuppressWarnings({"rawtypes", "unchecked"})
    private static String request(String line) {
        String[] request = line.split("\t", -1);
        if (request.length != 3 || !(request[1].equals("query") || request[1].equals("setup")))
            throw new IllegalArgumentException("request");
        String kind = request[1];
        yx codec = kind.equals("query") ? ModdedNetworkQueryPayload.STREAM_CODEC : ModdedNetworkPayload.STREAM_CODEC;
        if (request[0].equals("encode")) {
            vw buffer = new vw(Unpooled.buffer());
            try {
                codec.encode(buffer, model(kind, request[2]));
                byte[] bytes = new byte[buffer.readableBytes()];
                buffer.readBytes(bytes);
                return HEX.formatHex(bytes);
            } finally { buffer.release(); }
        }
        if (!request[0].equals("decode") || request[2].length() > 65534)
            throw new IllegalArgumentException("bounded decode request");
        vw buffer = new vw(Unpooled.wrappedBuffer(HEX.parseHex(request[2])));
        try {
            Object value = codec.decode(buffer);
            // Framing policy, not a property of StreamCodec.decode itself.
            if (buffer.isReadable()) throw new IllegalArgumentException("trailing bytes (local wrapper)");
            return canonical(kind, value);
        } finally { buffer.release(); }
    }
    public static void main(String[] args) throws Exception {
        if (Runtime.version().feature() != 21) throw new IllegalStateException("Java 21 required");
        // Avoid arbitrary log output mixing with the line protocol: responses have a prefix.
        BufferedReader input = new BufferedReader(new InputStreamReader(System.in, StandardCharsets.UTF_8));
        for (String line; (line = input.readLine()) != null;) {
            try {
                if (line.length() > 200000) throw new IllegalArgumentException("request size");
                System.out.println("REF\tOK\t" + request(line));
            } catch (RuntimeException | LinkageError error) {
                if (System.getenv("REFERENCE_DEBUG") != null) error.printStackTrace(System.err);
                System.out.println("REF\tERR\t" + error.getClass().getName());
            }
        }
    }
}
