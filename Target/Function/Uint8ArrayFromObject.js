var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Uint8ArrayFromObject_default = /* @__PURE__ */ __name(async (...[_Object]) => {
  const Keys = Object.keys(_Object).map(Number).sort((a, b) => a - b);
  const _Array = new Uint8Array(Keys.length);
  Keys.forEach((Key, Index) => {
    _Array[Index] = _Object[Key] ?? 0;
  });
  return _Array;
}, "default");
export {
  Uint8ArrayFromObject_default as default
};
//# sourceMappingURL=Uint8ArrayFromObject.js.map
