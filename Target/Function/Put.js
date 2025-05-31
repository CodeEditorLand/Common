var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Put_default = /* @__PURE__ */ __name(async (...[Instance]) => {
  if (Instance instanceof Map) {
    const _Value = {};
    for (const [Key, Value] of Instance.entries()) {
      if (Value instanceof Map) {
        _Value[Key] = await (await import("./Put.js")).default(Value);
      } else {
        _Value[Key] = Value;
      }
    }
    return _Value;
  }
  return Instance;
}, "default");
export {
  Put_default as default
};
//# sourceMappingURL=Put.js.map
