let u8mem;

let decoder = new TextDecoder('utf-8', {ignoreBOM: true, fatal: true});

function str_from_mem(ptr, len) {
    return decoder.decode(u8mem.slice(ptr, ptr + len));
}

onmessage = async function({data}) {
    const [mod, n, mem] = data;
    console.log('worker ' + n + ' started');

    u8mem = new Uint8Array(mem.buffer);

    const imports = {
        'env': {
            'memory': mem,
            '_logs': (ptr, len) => console.log(str_from_mem(ptr, len)),
            '_logi': (ptr, len, n) => console.log(str_from_mem(ptr, len) + ' ' + n),
        }
    };

    let inst = await WebAssembly.instantiate(mod, imports);

    if (inst.exports.initlock()) {
        inst.exports.__wasm_init_memory();
    }
    if (n == 0) {
        inst.exports.__wasm_init_tls(1024 * 1024 * 8);
    } else {
        inst.exports.__wasm_init_tls(1024 * 1024 * 8 + 1024 * 8);
        inst.exports.__sp.value += 1024 * 8;
    }

    inst.exports.start(n);
};
