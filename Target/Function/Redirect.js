var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });
var Redirect_default = /* @__PURE__ */ __name(async (...Option) => Response.redirect(
  Option[0] ?? "",
  Option[1] ?? 302
), "default");
const { Response } = await import("@cloudflare/workers-types/experimental/index.js");
export {
  Response,
  Redirect_default as default
};
//# sourceMappingURL=Redirect.js.map
