import initSync, { split_address } from '../pkg/wasm_sample.js';
import wasmModule from '../pkg/wasm_sample_bg.wasm';

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const address = url.searchParams.get("address");
    if(!address){
      return new Response(JSON.stringify({detail: "No address parameter."}), {
        status: 400,
        headers: { "Content-Type": "application/json" },
      });
    }

    try {
      await initSync(wasmModule);

      const res = {
          splitAddress: split_address(address)
      }

      return new Response(JSON.stringify(res), {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });
    } catch (err) {
      return new Response(`Error: ${err.message}`, { status: 500 });
    }
  },
};

