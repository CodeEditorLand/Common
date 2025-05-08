var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Encrypt_default = /* @__PURE__ */ __name(async (...[Data, Key]) => {
  const Vector = (await import("@cloudflare/workers-types/experimental")).crypto.getRandomValues(new Uint8Array(12));
  return {
    Vector,
    Data: new Uint8Array(
      await (await import("@cloudflare/workers-types/experimental")).crypto.subtle.encrypt(
        {
          name: "AES-GCM",
          iv: Vector,
          tagLength: 128
        },
        await (await import("@cloudflare/workers-types/experimental")).crypto.subtle.importKey(
          "jwk",
          {
            kty: "oct",
            k: Key,
            alg: "A256GCM",
            ext: true
          },
          {
            name: "AES-GCM"
          },
          false,
          ["encrypt", "decrypt"]
        ),
        (await import("buffer")).Buffer.from(JSON.stringify(Data)).buffer
      )
    )
  };
}, "default");
export {
  Encrypt_default as default
};
//# sourceMappingURL=Encrypt.js.map
