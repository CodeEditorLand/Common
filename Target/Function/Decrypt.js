var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Decrypt_default = /* @__PURE__ */ __name(async (...[Data, Key, Vector]) => new Uint8Array(
  await (await import("@cloudflare/workers-types/experimental")).crypto.subtle.decrypt(
    {
      name: "AES-GCM",
      iv: Vector,
      tagLength: 128
    },
    await (await import("@cloudflare/workers-types/experimental")).crypto.subtle.importKey(
      "jwk",
      {
        kty: "oct",
        k: Key ?? "",
        alg: "A256GCM",
        ext: true
      },
      {
        name: "AES-GCM"
      },
      false,
      ["encrypt", "decrypt"]
    ),
    Data
  )
), "default");
export {
  Decrypt_default as default
};
//# sourceMappingURL=Decrypt.js.map
