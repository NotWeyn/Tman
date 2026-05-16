

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.js";
export const imports = ["_app/immutable/nodes/0.Cw4haevW.js","_app/immutable/chunks/Bt-5c7_q.js","_app/immutable/chunks/Bkcb1oV-.js","_app/immutable/chunks/fhYyQWdB.js"];
export const stylesheets = ["_app/immutable/assets/0.DWcjfWhV.css"];
export const fonts = [];
