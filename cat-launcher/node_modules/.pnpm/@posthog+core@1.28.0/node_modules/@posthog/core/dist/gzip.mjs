function isGzipSupported() {
    return 'CompressionStream' in globalThis && 'TextEncoder' in globalThis && 'Response' in globalThis && 'function' == typeof Response.prototype.blob;
}
const isNativeAsyncGzipReadError = (error)=>{
    if (!error || 'object' != typeof error) return false;
    const name = 'name' in error ? String(error.name) : '';
    return 'NotReadableError' === name;
};
async function gzipCompress(input, isDebug = true, options) {
    try {
        const compressedStream = new CompressionStream('gzip');
        const writer = compressedStream.writable.getWriter();
        const writePromise = writer.write(new TextEncoder().encode(input)).then(()=>writer.close()).catch(async (err)=>{
            try {
                await writer.abort(err);
            } catch  {}
            throw err;
        });
        const responsePromise = new Response(compressedStream.readable).blob();
        const [compressed] = await Promise.all([
            responsePromise,
            writePromise
        ]);
        return compressed;
    } catch (error) {
        if (options?.rethrow) throw error;
        if (isDebug) console.error('Failed to gzip compress data', error);
        return null;
    }
}
export { gzipCompress, isGzipSupported, isNativeAsyncGzipReadError };
