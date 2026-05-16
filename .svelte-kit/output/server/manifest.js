export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","svelte.svg","tauri.svg","vite.svg"]),
	mimeTypes: {".png":"image/png",".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.Diva4xp8.js",app:"_app/immutable/entry/app.GpbxXtdu.js",imports:["_app/immutable/entry/start.Diva4xp8.js","_app/immutable/chunks/LGdyhWJr.js","_app/immutable/chunks/Bkcb1oV-.js","_app/immutable/chunks/CQq01EAv.js","_app/immutable/entry/app.GpbxXtdu.js","_app/immutable/chunks/Bkcb1oV-.js","_app/immutable/chunks/B6zQbD8H.js","_app/immutable/chunks/Bt-5c7_q.js","_app/immutable/chunks/CQq01EAv.js","_app/immutable/chunks/cJiEoT5t.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
