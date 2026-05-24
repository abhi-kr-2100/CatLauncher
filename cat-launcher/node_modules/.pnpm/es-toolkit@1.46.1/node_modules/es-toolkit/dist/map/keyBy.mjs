function keyBy(map, getKeyFromEntry) {
    const result = new Map();
    for (const [key, value] of map) {
        const newKey = getKeyFromEntry(value, key, map);
        result.set(newKey, value);
    }
    return result;
}

export { keyBy };
