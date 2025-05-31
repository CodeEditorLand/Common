var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Get_default = /* @__PURE__ */ __name(async (...[Instance]) => {
  if (typeof Instance === "string") {
    return Instance;
  }
  const _Map = /* @__PURE__ */ new Map();
  if (typeof Instance === "object") {
    for (const Key in Instance) {
      if (Object.prototype.hasOwnProperty.call(Instance, Key)) {
        if (typeof Instance[Key] === "object" && !Array.isArray(Instance[Key])) {
          _Map.set(
            Key,
            await (await import("./Get.js")).default(Instance[Key])
          );
        } else {
          _Map.set(Key, Instance[Key]);
        }
      }
    }
  }
  return _Map;
}, "default");
export {
  Get_default as default
};
//# sourceMappingURL=Get.js.map
