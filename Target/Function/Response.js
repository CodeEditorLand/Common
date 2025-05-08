var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Response_default = /* @__PURE__ */ __name(async (...Option) => new Response(JSON.stringify(Option[0]), {
  status: Option[1] ?? 200,
  headers: {
    "Content-Type": "application/json;charset=utf-8"
  }
}), "default");
const { Response } = await import("@cloudflare/workers-types/experimental/index.js");
export {
  Response,
  Response_default as default
};
//# sourceMappingURL=Response.js.map
